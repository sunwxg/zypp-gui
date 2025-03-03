use gtk::gio;
use gtk::gio::prelude::*;
use gtk::glib;
use log::debug;
use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet};
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
    //raw: String,
}

#[derive(Clone, Debug)]
pub struct RepoPackages {
    repo: String,
    file: String,
    enable: bool,
    //priority: i32,
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
    //search_entry: gtk::Entry,
}

impl PackageMeta {
    pub fn new() -> PackageMeta {
        let this = Self {
            data: Rc::new(RefCell::new(vec![])),
            sys_arch: PackageMeta::get_sys_arch(),
            //search_entry: search_entry,
        };

        this
    }

    pub fn update_data(&self) {
        let mut repo_package: Vec<RepoPackages> = vec![];
        self.read_dir(&mut repo_package);
        self.update_meta(&repo_package);
        self.data.replace(repo_package);
    }

    fn read_dir(&self, repo_package: &mut Vec<RepoPackages>) {
        let process = Command::new("find")
            .arg("/var/cache/zypp/raw")
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute rpm");

        let out = String::from_utf8_lossy(&process.stdout).to_string();

        for f in out.lines() {
            let mut _repo = String::new();
            let mut _enable = false;
            let mut _priority = 99;
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
                    _priority = v[1].parse::<i32>().unwrap();
                }

                repo_package.push(RepoPackages {
                    repo: _repo,
                    file: f.to_string(),
                    enable: _enable,
                    //priority: _priority,
                    meta: Rc::new(RefCell::new(HashMap::new())),
                    busy: Rc::new(RefCell::new(false)),
                });
            }
        }
    }

    fn update_meta(&self, repo_package: &Vec<RepoPackages>) {
        for r in repo_package {
            if !r.enable {
                continue;
            }
            let (tx, rx) = async_channel::bounded(1);

            r.set_busy(true);
            let file = r.file.clone();
            thread::spawn(move || {
                PackageMeta::read_file(file, tx);
            });

            let mut meta_hash: HashMap<String, Meta> = HashMap::new();
            let repo = r.clone();
            let this = self.clone();
            glib::spawn_future_local(async move {
                while let Ok(message) = rx.recv().await {
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
                }
            });
        }
    }

    fn read_file(file: String, sender: async_channel::Sender<Message>) {
        let command;
        let words: Vec<&str> = file.split('.').collect();
        match words.last() {
            Some(last_word) => match *last_word {
                "gz" => command = "gzip",
                "zst" => command = "zstd",
                _ => {
                    return;
                }
            },
            None => {
                return;
            }
        }

        let process = Command::new(command)
            .arg("-dc")
            .arg(file)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute rpm");
        let buffer = String::from_utf8_lossy(&process.stdout).to_string();

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
            //raw: String::new(),
        }
    }

    fn parse_line_by_line(buffer: String, sender: async_channel::Sender<Message>) {
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
                let message = data.clone();
                let tx = sender.clone();
                gio::spawn_blocking(move || {
                    tx.send_blocking(Message::Data(message))
                        .expect("Couldn't send data to channel");
                });

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
        let tx = sender.clone();
        gio::spawn_blocking(move || {
            tx.send_blocking(Message::Finish)
                .expect("Couldn't send data to channel");
        });
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
            let text = text.to_lowercase();
            let meta = repo.meta.borrow();
            for key in meta.keys() {
                let k = key.to_lowercase();
                match k.rfind(&text) {
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
        let process = Command::new("grep")
            .arg(fileld)
            .arg(file)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute rpm");

        String::from_utf8_lossy(&process.stdout).to_string()
    }

    fn get_sys_arch() -> String {
        let process = Command::new("uname")
            .arg("-m")
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute rpm");

        let out = String::from_utf8_lossy(&process.stdout).to_string();
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

        let col_types: [glib::Type; 1] = [glib::Type::STRING];
        let store = gtk::ListStore::new(&col_types);
        for i in hashset.drain() {
            store.set_value(&store.append(), 0 as u32, &i.to_value());
        }

        let entry_completion = gtk::EntryCompletion::new();
        entry_completion.set_text_column(0);
        entry_completion.set_minimum_key_length(3);
        entry_completion.set_popup_completion(true);
        entry_completion.set_model(Some(&store));

        //self.search_entry.set_completion(Some(&entry_completion));
    }
}
