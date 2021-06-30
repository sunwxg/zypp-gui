use gtk::prelude::*;
use log::debug;
use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::thread;

use crate::util::SearchInfo;

const HEAD: &'static str = "<package type=\"rpm\">";
const END: &'static str = "</package>";

pub enum Message {
    Finish,
    Data(Meta),
}

#[derive(Clone, Debug)]
pub struct Meta {
    name: String,
    arch: String,
    version: String,
    release: String,
    summary: String,
    description: String,
    location: String,
    //requires: Vec<String>,
    raw: String,
}

#[derive(Clone, Debug)]
pub struct RepoPackages {
    repo: String,
    file: String,
    enable: bool,
    priority: i32,
    meta: Rc<RefCell<HashMap<String, Meta>>>,
    busy: Rc<RefCell<bool>>,
}

impl RepoPackages {
    fn set_busy(&self, state: bool) {
        *self.busy.borrow_mut() = state;
    }

    fn busy(&self) -> bool {
        let state = self.busy.borrow();
        if *state {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug)]
pub struct PackageMeta {
    data: Rc<RefCell<Vec<RepoPackages>>>,
    sys_arch: String,
    search_entry: gtk::SearchEntry,
}

impl PackageMeta {
    pub fn new(search_entry: gtk::SearchEntry) -> PackageMeta {
        let this = Self {
            data: Rc::new(RefCell::new(vec![])),
            sys_arch: PackageMeta::get_sys_arch(),
            search_entry: search_entry,
        };

        this.update_data();
        this
    }

    pub fn update_data(&self) {
        let mut repo_package: Vec<RepoPackages> = vec![];
        self.read_dir(&mut repo_package);
        self.update_meta(&repo_package);

        self.data.replace(repo_package);
    }

    fn read_dir(&self, repo_package: &mut Vec<RepoPackages>) {
        let process = match Command::new("find")
            .arg("/var/cache/zypp/raw")
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(e) => panic!("failed spawn: {}", e),
            Ok(process) => process,
        };

        let mut out = String::new();
        match process.stdout.unwrap().read_to_string(&mut out) {
            Err(e) => panic!("couldn't read stdout: {}", e),
            Ok(_) => {}
        }

        for f in out.lines() {
            let mut _repo = String::new();
            let mut _enable = false;
            let mut priority = 99;
            if f.contains("primary") {
                let v: Vec<&str> = f
                    .strip_prefix("/var/cache/zypp/raw/")
                    .unwrap()
                    .split(|c| c == '/')
                    .collect();

                _repo = v[0].to_string();
                _enable = if self
                    .check_repo_state(_repo.clone(), "enable".to_string())
                    .contains("=1")
                {
                    true
                } else {
                    false
                };
                let value = self.check_repo_state(_repo.clone(), "priority".to_string());
                if value.len() > 0 && value.contains("priority=") {
                    let s = value.strip_suffix("\n").unwrap();
                    let v: Vec<&str> = s.split(|c| c == '=').collect();
                    priority = v[1].parse::<i32>().unwrap();
                }

                repo_package.push(RepoPackages {
                    repo: _repo,
                    file: f.to_string(),
                    enable: _enable,
                    priority: priority,
                    meta: Rc::new(RefCell::new(HashMap::new())),
                    busy: Rc::new(RefCell::new(false)),
                });
            }
        }
    }

    fn update_meta(&self, repo_package: &Vec<RepoPackages>) {
        for r in repo_package {
            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

            r.set_busy(true);
            let file = r.file.clone();
            thread::spawn(move || {
                PackageMeta::read_file(file, tx);
            });

            let mut meta_hash: HashMap<String, Meta> = HashMap::new();
            let repo = r.clone();
            let this = self.clone();
            rx.attach(None, move |message| {
                match message {
                    Message::Data(data) => {
                        meta_hash.insert(data.location.clone(), data.clone());
                    }
                    Message::Finish => {
                        debug!(
                            "Repo {} has {} packages",
                            repo.repo.clone(),
                            meta_hash.len()
                        );
                        repo.meta.replace(meta_hash.clone());
                        repo.set_busy(false);
                        this.add_package_name_set();
                    }
                }
                glib::Continue(true)
            });
        }
    }

    fn read_file(file: String, sender: glib::Sender<Message>) {
        let process = match Command::new("gzip")
            .arg("-dc")
            .arg(file)
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(e) => panic!("failed spawn: {}", e),
            Ok(process) => process,
        };

        let mut buffer = String::new();
        match process.stdout.unwrap().read_to_string(&mut buffer) {
            Err(e) => panic!("couldn't read stdout: {}", e),
            Ok(_) => {}
        }

        PackageMeta::parse_line_by_line(buffer, sender);
    }

    fn new_data() -> Meta {
        Meta {
            name: String::new(),
            arch: String::new(),
            version: String::new(),
            release: String::new(),
            summary: String::new(),
            description: String::new(),
            location: String::new(),
            raw: String::new(),
        }
    }

    fn parse_line_by_line(buffer: String, sender: glib::Sender<Message>) {
        let mut data = PackageMeta::new_data();
        let mut in_description = false;
        let mut head = true;
        let mut name = true;
        let mut arch = true;
        let mut summary = true;
        let mut description = true;
        let mut version = true;
        let mut location = true;

        for line in buffer.lines() {
            let line = line.trim_start();

            if head && line.starts_with(HEAD) {
                data = PackageMeta::new_data();
                //data.raw.push_str(line);
                //data.raw.push_str("\n");
                head = false;
                name = true;
                arch = true;
                summary = true;
                description = true;
                version = true;
                location = true;
                continue;
            }

            if line.starts_with(END) {
                //data.raw.push_str(line);
                sender
                    .send(Message::Data(data.clone()))
                    .expect("Couldn't send data to channel");

                head = true;
                continue;
            }

            if name && line.starts_with("<name>") {
                data.name = PackageMeta::get_tag(line, "<name>", "</name>");
                name = false;
                continue;
            }
            if arch && line.starts_with("<arch>") {
                data.arch = PackageMeta::get_tag(line, "<arch>", "</arch>");
                arch = false;
                continue;
            }
            if summary && line.starts_with("<summary>") {
                data.summary = PackageMeta::get_tag(line, "<summary>", "</summary>");
                summary = false;
                continue;
            }
            if description && line.starts_with("<description>") {
                data.description
                    .push_str(line.strip_prefix("<description>").unwrap());
                in_description = true;
                continue;
            }
            if description && line.ends_with("</description>") {
                data.description
                    .push_str(line.strip_suffix("</description>").unwrap());
                in_description = false;
                description = false;
                continue;
            }
            if version && line.starts_with("<version") {
                match PackageMeta::get_ver_rel(line) {
                    Some((v, r)) => {
                        data.version = v;
                        data.release = r;
                    }
                    None => {}
                }
                version = false;
                continue;
            }
            if location && line.starts_with("<location") {
                match PackageMeta::get_location(line) {
                    Some(v) => data.location = v,
                    None => {}
                }
                location = false;
                continue;
            }

            if in_description {
                data.description.push_str(line);
            }

            //data.raw.push_str(line);
            //data.raw.push_str("\n");
        }
        sender
            .send(Message::Finish)
            .expect("Couldn't send data to channel");
    }

    fn get_tag(line: &str, tag: &str, end: &str) -> String {
        let s = line.strip_prefix(tag).unwrap();
        s.strip_suffix(end).unwrap().to_string()
    }

    fn get_ver_rel(line: &str) -> Option<(String, String)> {
        let s: Vec<&str> = line.rsplit(|c| c == ' ').collect();
        let ver: Vec<&str> = s
            .iter()
            .find(|c| c.trim().starts_with("ver="))
            .unwrap()
            .split(|c| c == '"')
            .collect();
        let rel: Vec<&str> = s
            .iter()
            .find(|c| c.trim().starts_with("rel="))
            .unwrap()
            .split(|c| c == '"')
            .collect();
        Some((ver[1].to_string(), rel[1].to_string()))
    }

    fn get_location(line: &str) -> Option<String> {
        let s: Vec<&str> = line.rsplit(|c| c == ' ').collect();
        let v: Vec<&str> = s
            .iter()
            .find(|c| c.trim().starts_with("href="))
            .unwrap()
            .split(|c| c == '"')
            .collect();
        Some(v[1].to_string())
    }

    pub fn search(&self, text: String) -> Vec<SearchInfo> {
        let mut result: Vec<SearchInfo> = vec![];
        let meta: Ref<Vec<RepoPackages>> = self.data.borrow();
        for repo in meta.clone().into_iter() {
            if repo.busy() {
                continue;
            }
            if !repo.enable {
                continue;
            }
            let meta = repo.meta.borrow();
            for key in meta.keys() {
                match key.rfind(&text) {
                    Some(_) => {
                        let arch: Vec<&str> = key.split(|c| c == '/').collect();
                        if arch[0] != self.sys_arch && arch[0] != "noarch" {
                            continue;
                        }
                        let d = meta.get(key).unwrap();
                        let info = self.check_installed(d.name.clone());
                        let mut _id = String::new();
                        if info == "installed" {
                            _id = format!(
                                "{};{}-{};{};{}",
                                d.name.clone(),
                                d.version,
                                d.release,
                                d.arch,
                                "installed"
                            );
                        } else {
                            _id = format!(
                                "{};{}-{};{};{}",
                                d.name.clone(),
                                d.version,
                                d.release,
                                d.arch,
                                repo.repo
                            );
                        }
                        result.push(SearchInfo {
                            name: d.name.clone(),
                            id: _id,
                            summary: d.summary.clone(),
                            info: info,
                        });
                    }
                    None => {}
                }
            }
        }
        result.sort_by(|a, b| a.name.cmp(&b.name));
        result
    }

    fn check_repo_state(&self, repo: String, fileld: String) -> String {
        let file = format!("/etc/zypp/repos.d/{}.repo", repo);
        let process = match Command::new("grep")
            .arg(fileld)
            .arg(file)
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(e) => panic!("failed spawn: {}", e),
            Ok(process) => process,
        };

        let mut out = String::new();
        match process.stdout.unwrap().read_to_string(&mut out) {
            Err(e) => panic!("couldn't read stdout: {}", e),
            Ok(_) => {}
        }

        out
    }

    fn get_sys_arch() -> String {
        let process = match Command::new("uname")
            .arg("-m")
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(e) => panic!("failed spawn: {}", e),
            Ok(process) => process,
        };

        let mut out = String::new();
        match process.stdout.unwrap().read_to_string(&mut out) {
            Err(e) => panic!("couldn't read stdout: {}", e),
            Ok(_) => {}
        }

        out.strip_suffix("\n").unwrap().to_string()
    }

    fn check_installed(&self, name: String) -> String {
        let status = Command::new("rpm")
            .arg("-qi")
            .arg(name)
            .stdout(Stdio::piped())
            .status()
            .expect("failed to execute rpm");

        if status.success() {
            "installed".to_string()
        } else {
            "".to_string()
        }
    }

    fn add_package_name_set(&self) {
        let meta: Ref<Vec<RepoPackages>> = self.data.borrow();
        for repo in meta.clone().into_iter() {
            if repo.busy() {
                return;
            }
        }
        let mut hashset: HashSet<String> = HashSet::new();
        for repo in meta.clone().into_iter() {
            if repo.enable {
                let meta = repo.meta.borrow();
                for v in meta.values() {
                    hashset.insert(v.name.clone());
                }
            }
        }

        let col_types: [glib::Type; 1] = [glib::Type::String];
        let store = gtk::ListStore::new(&col_types);
        for i in hashset.drain() {
            store.set_value(&store.append(), 0 as u32, &i.to_value());
        }

        let entry_completion = gtk::EntryCompletion::new();
        entry_completion.set_text_column(0);
        entry_completion.set_minimum_key_length(3);
        entry_completion.set_popup_completion(true);
        entry_completion.set_model(Some(&store));

        self.search_entry.set_completion(Some(&entry_completion));
    }
}
