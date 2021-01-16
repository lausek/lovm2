pub(crate) mod buffer;
pub(crate) mod collection;
pub(crate) mod data;
pub(crate) mod fs;
pub(crate) mod functional;
pub(crate) mod json;
pub(crate) mod math;
#[cfg(feature = "net")]
pub(crate) mod net;
pub(crate) mod regex;
pub(crate) mod string;

pub use self::buffer::*;
pub use self::collection::*;
pub use self::data::*;
pub use self::fs::*;
pub use self::functional::*;
pub use self::json::*;
pub use self::math::*;
#[cfg(feature = "net")]
pub use self::net::*;
pub use self::regex::*;
pub use self::string::*;

use lovm2_extend::prelude::*;

macro_rules! add_function {
    ($module:expr, $name:ident) => {
        $module
            .slots
            .insert(stringify!($name), create_callable($name))
    };
}

/// Add standard functions to the given vm. If `create_vm_with_std` is used, this
/// gets loaded automatically.
pub fn create_std_module() -> Module {
    let mut module = ModuleBuilder::with_meta(ModuleMeta::new("std".to_string(), None, vec![]));
    let mut module = module.build().unwrap();

    add_function!(module, absolute);
    add_function!(module, acos);
    add_function!(module, all);
    add_function!(module, any);
    add_function!(module, append);
    add_function!(module, argn);
    add_function!(module, asin);
    add_function!(module, atan);
    add_function!(module, atan2);
    add_function!(module, basename);
    add_function!(module, call);
    add_function!(module, captures);
    add_function!(module, ceil);
    add_function!(module, chr);
    add_function!(module, clamp);
    add_function!(module, contains);
    add_function!(module, cos);
    add_function!(module, create_file);
    add_function!(module, decode);
    add_function!(module, deep_clone);
    add_function!(module, delete);
    add_function!(module, e);
    add_function!(module, encode);
    add_function!(module, exists);
    add_function!(module, filter);
    add_function!(module, floor);
    add_function!(module, format);
    add_function!(module, get);
    add_function!(module, has_data);
    add_function!(module, index_of);
    add_function!(module, input);
    add_function!(module, is_dir);
    add_function!(module, is_match);
    add_function!(module, join);
    add_function!(module, len);
    add_function!(module, list_dir);
    add_function!(module, log);
    add_function!(module, map);
    add_function!(module, mkdir);
    add_function!(module, new_buffer);
    add_function!(module, new_regex);
    add_function!(module, open_file);
    add_function!(module, ord);
    add_function!(module, parent);
    add_function!(module, pi);
    add_function!(module, pop_vstack);
    add_function!(module, print);
    add_function!(module, push_vstack);
    add_function!(module, read_all);
    add_function!(module, read_line);
    add_function!(module, readn);
    add_function!(module, rename);
    add_function!(module, replace);
    add_function!(module, rmdir);
    add_function!(module, round);
    add_function!(module, set);
    add_function!(module, sin);
    add_function!(module, sort);
    add_function!(module, split);
    add_function!(module, sqrt);
    add_function!(module, tan);
    add_function!(module, to_lower);
    add_function!(module, to_upper);
    add_function!(module, trim);
    add_function!(module, unlink);
    add_function!(module, write_all);
    add_function!(module, writes);

    if cfg!(feature = "net") {
        add_function!(module, exec);
        add_function!(module, get_body_as_buffer);
        add_function!(module, get_body_as_string);
        add_function!(module, get_url);
        add_function!(module, get_status);
        add_function!(module, new_request);
        add_function!(module, serve);
        add_function!(module, set_body);
        add_function!(module, set_header);
        add_function!(module, set_method);
    }

    module
}

pub fn input(vm: &mut Vm) -> Lovm2Result<()> {
    use std::io::stdin;

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    vm.context_mut().push_value(Value::Str(input));

    Ok(())
}

#[lovm2_function]
fn len(val: &Value) -> Lovm2Result<i64> {
    val.as_any_inner()
        .and_then(|any| {
            if let Some(buf) = any.borrow().0.downcast_ref::<Buffer>() {
                return Ok(buf.inner.len() as i64);
            }

            if let Some(file) = any.borrow().0.downcast_ref::<File>() {
                let meta = file.inner.metadata().or_else(err_from_string)?;
                return Ok(meta.len() as i64);
            }

            err_method_not_supported("count")
        })
        .or_else(|_| val.len().map(|n| n as i64))
}

pub fn print(vm: &mut Vm) -> Lovm2Result<()> {
    use std::io::Write;

    let argn = vm.context_mut().frame_mut().unwrap().argn;
    let mut args: Vec<String> = (0..argn)
        .map(|_| vm.context_mut().pop_value().unwrap())
        .map(|x| format!("{}", x))
        .collect();

    args.reverse();

    print!("{}", args.join(" "));
    std::io::stdout().flush().unwrap();
    vm.context_mut().push_value(Value::Nil);

    Ok(())
}
