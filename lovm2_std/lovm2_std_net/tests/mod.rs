#![cfg(test)]

use lovm2::prelude::*;
use lovm2_extend::prelude::*;

use httptest::{all_of, matchers::request, responders::*, Expectation, ServerPool};

static SERVER_POOL: ServerPool = ServerPool::new(2);

fn run_module_test(func: impl Fn(&mut ModuleBuilder)) -> Vm {
    let mut builder = ModuleBuilder::new();
    builder
        .entry()
        .step(Include::import_global("liblovm2_std_net"));
    func(&mut builder);
    let module = builder.build().unwrap();

    let mut vm = create_test_vm();
    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    vm
}

#[test]
fn get_body_as_string() {
    let server = SERVER_POOL.get_server();
    server.expect(
        Expectation::matching(all_of![
            request::method("GET"),
            request::path("/simple-get"),
        ])
        .respond_with(json_encoded(vec!["a", "b"])),
    );

    let mut vm = run_module_test(|_| {});

    let host = server.url("/simple-get").to_string();
    let req = vm.call("new_request", &[host.into()]).unwrap();
    let res = vm.call("exec", &[req]).unwrap();
    let body = vm.call("get_body_as_string", &[res.clone()]).unwrap();

    assert_eq!(Value::from(200), vm.call("get_status", &[res]).unwrap());
    assert_eq!(Value::from("[\"a\",\"b\"]"), body);
}

#[test]
fn post_request() {
    let server = SERVER_POOL.get_server();
    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/receive-data"),
            request::body("abcd"),
        ])
        .respond_with(status_code(200)),
    );

    let mut vm = run_module_test(|_| {});

    let host = server.url("/receive-data").to_string();
    let req = vm.call("new_request", &[host.into()]).unwrap();
    vm.call("set_method", &[req.clone(), "pOsT".into()])
        .unwrap();
    vm.call("set_body", &[req.clone(), "abcd".into()]).unwrap();
    let res = vm.call("exec", &[req]).unwrap();

    assert_eq!(Value::from(200), vm.call("get_status", &[res]).unwrap());
}

#[test]
fn error_status_code() {
    let server = SERVER_POOL.get_server();
    server.expect(
        Expectation::matching(all_of![
            request::method("GET"),
            request::path("/receive-data"),
        ])
        .respond_with(status_code(403)),
    );

    let mut vm = run_module_test(|_| {});

    let host = server.url("/receive-data").to_string();
    let req = vm.call("new_request", &[host.into()]).unwrap();
    let res = vm.call("exec", &[req]).unwrap();

    assert_eq!(Value::from(403), vm.call("get_status", &[res]).unwrap());
}
