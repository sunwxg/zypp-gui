use log::debug;
use std::process::{Command, Stdio};

#[derive(Clone)]
pub enum Settings {
    Enable(bool),
    Refresh(bool),
    Priority(i32),
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct RepoInfo {
    pub id: String,
    pub alias: String,
    pub name: String,
    pub enable: bool,
    pub cpg: bool,
    pub refresh: bool,
    pub priority: i32,
    pub url: String,
}

pub struct Zypper {}

impl Zypper {
    fn to_repoinfo(line: &str) -> RepoInfo {
        let r: Vec<&str> = line.split("|").collect();
        let id = r[0].trim().to_string();
        let alias = r[1].trim().to_string();
        let name = r[2].trim().to_string();
        let enable = if r[3].trim().to_string().contains("Yes") {
            true
        } else {
            false
        };
        let cpg = if r[4].trim().to_string().contains("Yes") {
            true
        } else {
            false
        };
        let refresh = if r[5].trim().to_string().contains("Yes") {
            true
        } else {
            false
        };
        let priority = r[7].trim().to_string().parse::<i32>().unwrap();
        let url = r[9].trim().to_string();

        RepoInfo {
            id: id,
            alias: alias,
            name: name,
            enable: enable,
            cpg: cpg,
            refresh: refresh,
            priority: priority,
            url: url,
        }
    }

    pub fn get_repos() -> Option<Vec<RepoInfo>> {
        let process = Command::new("zypper")
            .arg("lr")
            .arg("-d")
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute rpm");

        let s = String::from_utf8_lossy(&process.stdout).to_string();
        let v: Vec<&str> = s.split("\n").collect();
        if v.len() < 3 {
            return None;
        }
        let mut repos: Vec<RepoInfo> = vec![];
        for i in 2..v.len() - 1 {
            repos.push(Self::to_repoinfo(v[i]));
        }
        return Some(repos);
    }

    pub fn change_repo(id: String, settings: Settings) -> bool {
        let mut args: Vec<&str> = vec![];
        let mut _value = String::new();
        match settings {
            Settings::Enable(s) => args.push(if s { "-e" } else { "-d" }),
            Settings::Refresh(s) => args.push(if s { "-r" } else { "-n" }),
            Settings::Priority(s) => {
                args.push("-p");
                _value = s.to_string();
                args.push(&_value);
            }
        }

        let child = Command::new("pkexec")
            .arg("mod-repo")
            .args(args)
            .arg(id)
            .spawn()
            .expect("failed to run pkexec");

        let output = child.wait_with_output().expect("fail to wait pkexec");
        if !output.status.success() {
            debug!("pkexec fail");
            return false;
        } else {
            return true;
        }
    }

    pub fn add_repo(name: String, url: String) -> bool {
        let child = Command::new("pkexec")
            .arg("mod-repo")
            .arg("add")
            .arg("--name")
            .arg(name)
            .arg("--url")
            .arg(url)
            .spawn()
            .expect("failed to run pkexec");

        let output = child.wait_with_output().expect("fail to wait pkexec");
        if !output.status.success() {
            debug!("pkexec fail");
            return false;
        } else {
            return true;
        }
    }

    pub fn delete_repo(id: String) -> bool {
        let child = Command::new("pkexec")
            .arg("mod-repo")
            .arg("delete")
            .arg("--id")
            .arg(id)
            .spawn()
            .expect("failed to run pkexec");

        let output = child.wait_with_output().expect("fail to wait pkexec");
        if !output.status.success() {
            debug!("pkexec fail");
            return false;
        } else {
            return true;
        }
    }
}

#[test]
fn get_repos() {
    Zypper::get_repos();
}
