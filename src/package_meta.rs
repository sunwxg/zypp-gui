use std::collections::HashMap;
use std::process::{Command, Stdio};
//use std::fs::File;
use std::cell::{Ref, RefCell};
use std::io::prelude::*;
use std::rc::Rc;

use crate::util::{SearchInfo};

const HEAD: &'static str = "<package type=\"rpm\">";
const END: &'static str = "</package>";

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
    meta: Option<HashMap<String, Meta>>,
}

#[derive(Clone, Debug)]
pub struct PackageMeta {
    data: Rc<RefCell<Vec<RepoPackages>>>,
    sys_arch: String,
}

impl PackageMeta {
    pub fn new() -> PackageMeta {
        let this = Self {
            data: Rc::new(RefCell::new(vec![])),
            sys_arch: PackageMeta::get_sys_arch(),
        };

        this.build_data();
        this
    }

    fn build_data(&self) {
        let mut repo_package: Vec<RepoPackages> = vec![];
        self.read_dir(&mut repo_package);

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
                _enable = if self.check_repo_state(_repo.clone(), "enable".to_string()).contains("=1") { true } else { false };
                let value = self.check_repo_state(_repo.clone(), "priority".to_string());
                if value.len() > 0 && value.contains("priority=") {
                    let s = value.strip_suffix("\n").unwrap();
                    let v: Vec<&str> = s.split(|c| c == '=' ).collect();
                    priority = v[1].parse::<i32>().unwrap();
                }

                repo_package.push(RepoPackages {
                    repo: _repo,
                    file: f.to_string(),
                    enable: _enable,
                    priority: priority,
                    meta: None,
                });
            }
        }

        for mut r in repo_package {
            r.meta = Some(self.read_file(r.file.clone()));
        }
    }

    fn read_file(&self, file: String) -> HashMap<String, Meta> {
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

        self.parse_line_by_line(buffer)
    }

    fn new_data(&self) -> Meta {
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

    fn parse_line_by_line(&self, buffer: String) -> HashMap<String, Meta> {
        let mut meta_hash: HashMap<String, Meta> = HashMap::new();

        let mut data = self.new_data();
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
                data = self.new_data();
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
                meta_hash.insert(data.location.clone(), data.clone());
                head = true;
                continue;
            }

            if name && line.starts_with("<name>") {
                data.name = self.get_tag(line, "<name>", "</name>");
                name = false;
                continue;
            }
            if arch && line.starts_with("<arch>") {
                data.arch = self.get_tag(line, "<arch>", "</arch>");
                arch = false;
                continue;
            }
            if summary && line.starts_with("<summary>") {
                data.summary = self.get_tag(line, "<summary>", "</summary>");
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
                match self.get_ver_rel(line) {
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
                match self.get_location(line) {
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
        println!("len={}", meta_hash.len());
        meta_hash
    }

    fn get_tag(&self, line: &str, tag: &str, end: &str) -> String {
        let s = line.strip_prefix(tag).unwrap();
        s.strip_suffix(end).unwrap().to_string()
    }

    fn get_ver_rel(&self, line: &str) -> Option<(String, String)> {
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

    fn get_location(&self, line: &str) -> Option<String> {
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
            if !repo.enable {
                continue;
            }
            let meta = repo.meta.unwrap();
            for key in meta.keys() {
                match key.rfind(&text) {
                    Some(_) => {
                        let arch: Vec<&str> = key.split(|c| c == '/' ).collect();
                        if arch[0] != self.sys_arch && arch[0] != "noarch" {
                            continue;
                        }
                        let d = meta.get(key).unwrap();
                        let info = self.check_installed(d.name.clone());
                        let mut _id = String::new();
                        if info == "installed" {
                            _id =  format!( "{};{}-{};{};{}", d.name.clone(), d.version, d.release, d.arch, "installed");
                        } else {
                            _id =  format!( "{};{}-{};{};{}", d.name.clone(), d.version, d.release, d.arch, repo.repo);
                        }
                        result.push(
                            SearchInfo {
                            name: d.name.clone(),
                            id: _id,
                            summary: d.summary.clone(),
                            info: info,
                            });
                    }
                    None => {}
                }}}
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
            Ok(_) => {},
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
            .status()
            .expect("failed to execute rpm");

        if status.success() {
            "installed".to_string()
        } else {
            "".to_string()
        }
    }
}
