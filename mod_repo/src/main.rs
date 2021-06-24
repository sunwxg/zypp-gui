extern crate clap;
use clap::{App, Arg};
use std::process::{exit, Command, Stdio};

fn main() {
    let matches = App::new("Modify the repo settings")
        .version("1.0")
        .arg(
            Arg::with_name("enable")
                .short("e")
                .long("enable")
                .help("Enable the repo"),
        )
        .arg(
            Arg::with_name("disable")
                .short("d")
                .long("disable")
                .help("Disable the repo"),
        )
        .arg(
            Arg::with_name("refresh")
                .short("r")
                .long("refresh")
                .help("Refresh the repo"),
        )
        .arg(
            Arg::with_name("no-refresh")
                .short("n")
                .long("no-refresh")
                .help("Not refresh the repo"),
        )
        .arg(
            Arg::with_name("priority")
                .short("p")
                .long("priority")
                .help("Priority of the repo")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("cpg")
                .short("g")
                .long("cpg")
                .help("CPG check"),
        )
        .arg(
            Arg::with_name("no-cpg")
                .short("G")
                .long("no-cpg")
                .help("no CPG check"),
        )
        .arg(
            Arg::with_name("repo")
                .help("repo id")
                .required(true)
                .index(1),
        )
        .get_matches();

    let mut args: Vec<&str> = vec![];
    if matches.is_present("enable") {
        args.push("-e");
    } else if matches.is_present("disable") {
        args.push("-d");
    } else if matches.is_present("refresh") {
        args.push("-r");
    } else if matches.is_present("no-refresh") {
        args.push("-n");
    } else if matches.is_present("cpg") {
        args.push("-g");
    } else if matches.is_present("no-cpg") {
        args.push("-G");
    } else if matches.is_present("priority") {
        let value = matches.value_of("priority").unwrap();
        args.push("-p");
        args.push(value);
    }

    let id = matches.value_of("repo").unwrap();
    args.push(id);

    let process = match Command::new("zypper")
        .arg("mr")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(e) => panic!("failed spawn zypper: {}", e),
        Ok(process) => process,
    };

    match process.stderr {
        Some(_) => {
            exit(1);
        }
        None => {}
    }
}
