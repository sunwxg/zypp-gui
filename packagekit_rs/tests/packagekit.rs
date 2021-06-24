//use pk_rs::client;
use packagekit_rs::*;

#[test]
fn client_new() {
    let _client = ClientPk::new();
}

#[test]
fn task_new() {
    let _task = Task::new();
}

#[test]
fn refresh_cache() {
    let client = ClientPk::new();
    client.refresh_cache(true).expect("fail refresh cache");
}

#[test]
fn get_updates() {
    let client = ClientPk::new();
    let result = client.get_updates(None).expect("fail updates");
    let vecc = result.package_array();
    assert!(vecc.len() > 0, "get 0 updates");
}

#[test]
fn get_packages() {
    let client = ClientPk::new();
    let result = client.get_packages(None).expect("fail updates");
    let vecc = result.package_array();
    assert!(vecc.len() > 0, "get 0 packages");
}
