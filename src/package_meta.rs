use std::collections::HashMap;
use std::process::{Command, Stdio};
//use std::fs::File;
use std::cell::{Ref, RefCell};
use std::io::prelude::*;
use std::rc::Rc;

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
    meta: Option<HashMap<String, Meta>>,
}

#[derive(Clone, Debug)]
pub struct PackageMeta {
    data: Rc<RefCell<Vec<RepoPackages>>>,
}

impl PackageMeta {
    pub fn new() -> PackageMeta {
        let this = Self {
            data: Rc::new(RefCell::new(vec![])),
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
            if f.contains("primary") {
                let v: Vec<&str> = f
                    .strip_prefix("/var/cache/zypp/raw/")
                    .unwrap()
                    .split(|c| c == '/')
                    .collect();
                repo_package.push(RepoPackages {
                    repo: v[0].to_string(),
                    file: f.to_string(),
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
                if meta_hash.contains_key(&data.location) {
                    println!("{}", data.location.clone());
                }
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
        //for p in meta_hash.values() {
        //println!("{}", p.name.clone());
        //}
        //println!("{}", meta_vec[1].raw);
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

    pub fn search(&self, text: String) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let meta: Ref<Vec<RepoPackages>> = self.data.borrow();
        for repo in meta.clone().into_iter() {
            let meta = repo.meta.unwrap();
            for key in meta.keys() {
                match key.rfind(&text) {
                    Some(_) => {
                        let d = meta.get(key).unwrap();
                        result.push(format!(
                            "{};{};{}-{};{}",
                            repo.repo, d.name, d.version, d.release, d.summary
                        ));
                    }
                    None => {}
                }
            }
        }
        result
    }
}
