#[test]
fn to_lower_camel_case() {
    use lovm2::prelude::*;

    let tlcc = to_lower_camel_case;

    assert_eq!(tlcc(""), "");
    assert_eq!(tlcc("call"), "call");
    assert_eq!(tlcc("call_extern"), "callExtern");
    assert_eq!(tlcc("_call_extern"), "_callExtern");
    assert_eq!(tlcc("call_Extern"), "callExtern");
    assert_eq!(tlcc("CALL_EXTERN"), "callExtern");
}
