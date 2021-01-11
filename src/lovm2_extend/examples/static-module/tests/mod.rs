#![cfg(test)]

use lovm2_extend::prelude::*;

use static_module::*;

#[test]
fn function_is_present() {
    let mut cvm = CustomVm::new();

    assert_eq!(
        Value::from(2),
        cvm.inner().call("minus", &[5.into(), 3.into()]).unwrap(),
    );
}
