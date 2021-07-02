extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
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
                //.required(true)
                .index(1),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Add repo")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .help("repo name")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("url")
                        .short("u")
                        .long("url")
                        .help("repo url")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete").about("Delete repo").arg(
                Arg::with_name("id")
                    .short("i")
                    .long("id")
                    .help("repo id")
                    .required(true)
                    .takes_value(true),
            ),
        )
        .get_matches();

    if matches.is_present("add") {
        add_repo(matches);
    } else if matches.is_present("delete") {
        delete_repo(matches);
    } else {
        modify_repo(matches);
    }
}

fn modify_repo(matches: ArgMatches) {
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

    let status = Command::new("zypper")
        .arg("mr")
        .args(args)
        .stdout(Stdio::piped())
        .status()
        .expect("failed to execute zypper");

    if status.success() {
        exit(0);
    } else {
        exit(1);
    }
}

fn add_repo(matches: ArgMatches) {
    let mut args: Vec<&str> = vec![];
    let add_matches = matches.subcommand_matches("add").unwrap();

    if add_matches.is_present("url") {
        let value = add_matches.value_of("url").unwrap();
        args.push(value);
    }
    if add_matches.is_present("name") {
        let value = add_matches.value_of("name").unwrap();
        args.push(value);
    }

    let status = Command::new("zypper")
        .arg("ar")
        .arg("-d")
        .arg("-f")
        .args(args)
        .stdout(Stdio::piped())
        .status()
        .expect("failed to execute zypper");

    if status.success() {
        exit(0);
    } else {
        exit(1);
    }
}

fn delete_repo(matches: ArgMatches) {
    let mut args: Vec<&str> = vec![];
    let delete_matches = matches.subcommand_matches("delete").unwrap();

    if delete_matches.is_present("id") {
        let value = delete_matches.value_of("id").unwrap();
        args.push(value);
    }

    let status = Command::new("zypper")
        .arg("rr")
        .args(args)
        .stdout(Stdio::piped())
        .status()
        .expect("failed to execute zypper");

    if status.success() {
        exit(0);
    } else {
        exit(1);
    }
}
