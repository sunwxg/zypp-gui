extern crate minidom;
extern crate serde;
extern crate serde_json;
extern crate clap;

use std::collections::HashMap;
use std::io::prelude::*;
use minidom::Element;
use serde::{Deserialize, Serialize};
use clap::{App, Arg};

mod name_convert;
use crate::name_convert::NAME_CONVERT;

const NS: &'static str = "metadata/common";
const NSHEAD: &'static str = r#"<metadata xmlns="metadata/common" xmlns:rpm="metadata/rpm">"#;
const NSEND: &'static str = r#"</metadata>"#;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data {
    asia: Vec<Site>,
    africa: Vec<Site>,
    europe: Vec<Site>,
    north_america: Vec<Site>,
    oceania: Vec<Site>,
    south_america: Vec<Site>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Site {
    pub country: String,
    pub name: String,
    pub http: String,
    pub rsync: String,
    pub ftp: String,
}

fn main() {
    let matches = App::new("Get openSUSE mirror site data")
        .version("1.0")
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("get the number of site"),
        )
        .get_matches();
    let mut print_number = false;
    if matches.is_present("number") {
        print_number = true;
    }

    let mut name_map: HashMap<String, String> = HashMap::new();
    for i in NAME_CONVERT {
        name_map.insert(i.0.to_string(), i.1.to_string());
    }

    let mut buffer = String::new();
    let _ = std::io::stdin().read_to_string(&mut buffer);

    let data = get_data(buffer);

    let root: Element = data.parse().unwrap();

    let body = root.get_child("table",NS).unwrap()
        .get_child("tbody", NS).unwrap();

    let mut  _map: HashMap<String, Vec<Site>> = HashMap::new();
    let mut  _region: Vec<Site> = vec![];
    let mut  _region_name = String::new();

    for c in body.children() {
        let (is_region, skip) = is_region(c);
        if is_region {
            if _region.len() > 0 {
                _map.insert(_region_name, _region);
            }
            _region_name = get_region(c);
            _region = vec![];
            continue;
        }
        if skip {
            continue;
        }

        _region.push(get_site(c, name_map.clone()));
    }
    _map.insert(_region_name, _region);
    if print_number {
        for i in _map.clone() {
            println!("{} {}", i.0, i.1.len());
        }
    }

    let result = Data {
        asia: _map.get("Asia:").unwrap().clone(),
        africa: _map.get("Africa:").unwrap().clone(),
        europe: _map.get("Europe:").unwrap().clone(),
        north_america: _map.get("North America:").unwrap().clone(),
        oceania: _map.get("Oceania:").unwrap().clone(),
        south_america: _map.get("South America:").unwrap().clone(),
    };

    if !print_number {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    }
}

fn get_site(e: &Element, name_map: HashMap<String, String>) -> Site {
    let mut country = String::new();
    let mut name = String::new();
    let mut http = (String::new(), String::new());
    let mut rsync = (String::new(), String::new());
    let mut ftp = (String::new(), String::new());

    for c1 in e.children() {
        if c1.is("td", NS) {
            for c2 in c1.children() {
                if c2.is("img", NS) {
                    country = c1.text().trim().to_string();
                }
                if c2.is("a", NS) {
                    match c2.text().as_str() {
                        "HTTP" => { http = ("HTTP".to_string(), c2.attr("href").unwrap().to_string()); },
                        "rsync" => { rsync = ("rsync".to_string(), c2.attr("href").unwrap().to_string()); },
                        "FTP" => { ftp = ("ftp".to_string(), c2.attr("href").unwrap().to_string()); },
                        _ => {
                            name = match name_map.get(c2.text().as_str()) {
                                Some(name) => name.to_string(),
                                None => c2.text(),
                            };
                        },
                    }
                }
            }
        }
    }

    let site = Site {
        country: country,
        name: name,
        http: http.1,
        rsync: rsync.1,
        ftp: ftp.1,
    };
    site
}

fn is_region(e: &Element) -> (bool, bool) {
    let mut _ret = false;
    let mut _skip = false;
    for c1 in e.children() {
        if c1.is("td", NS) {
            match c1.attr("colspan") {
                Some(_) => {
                    if c1.attr("colspan").unwrap() == "28" {
                        _ret = true;
                    } else if c1.attr("colspan").unwrap() == "6" {
                        _skip = true;
                    }
                },
                None => {},
            }
        }
    }
    (_ret, _skip)
}

fn get_region(e: &Element) -> String {
    let mut _region = String::new();
    let td = e.get_child("td", NS).unwrap();
    match td.attr("colspan") {
        Some(_) => {
            _region = td.text();
        },
        None => {},
    }
    _region
}

fn get_data(buffer: String) -> String {
    let mut data = String::new();
    data.push_str(NSHEAD);

    let mut _in_table = false;
    for line in buffer.lines() {
        if line.contains("<table>") {
            _in_table = true;
            data.push_str(line);
            continue;
        }
        if line.contains("</table>") {
            _in_table = false;
            data.push_str(line);
            data.push_str(NSEND);
            continue;
        }

        if line.len() == 0 {
            continue;
        }
        if line.contains("radic") {
            continue;
        }

        if _in_table {
            data.push_str(line);
        }
    }

    data
}
