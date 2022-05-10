#![cfg(test)]

use lovm2::{prelude::*, vm::LV2LoadRequest};
use lovm2_sexp::create_module;

pub fn create_runtime(name: &str, src: &str) -> Interpreter {
    let mut int = Interpreter::new();
    let module = create_module(name, src).unwrap();

    println!("{}", module);

    int.load_global(module).unwrap();
    int
}

fn load_hook(_req: &LV2LoadRequest) -> LV2Result<Option<LV2Module>> {
    Ok(None)
}

fn import_hook(module: Option<&str>, name: &str) -> LV2Result<Option<String>> {
    let name = name.replace("_", "-");
    let name = match module {
        Some(module) => format!("{}-{}", module, name),
        _ => name.to_string(),
    };
    Ok(Some(name))
}

pub struct Interpreter {
    vm: LV2Vm,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut vm = lovm2::create_vm_with_std();

        vm.set_load_hook(load_hook);
        vm.set_import_hook(import_hook);

        Self { vm }
    }

    pub fn call<T>(&mut self, name: &str, args: &[T]) -> LV2Result<LV2Value>
    where
        T: Into<LV2Value> + Clone,
    {
        let args: Vec<LV2Value> = args.iter().map(T::clone).map(T::into).collect();
        self.vm.call(name, args.as_ref())
    }

    pub fn load(&mut self, module: LV2Module) -> LV2Result<()> {
        // import module namespaced
        self.vm.add_module(module, true)
    }

    pub fn load_global(&mut self, module: LV2Module) -> LV2Result<()> {
        // import module namespaced
        self.vm.add_module(module, false)
    }

    pub fn load_main(&mut self, module: LV2Module) -> LV2Result<()> {
        self.vm.add_main_module(module)
    }

    pub fn run(&mut self) -> LV2Result<LV2Value> {
        self.vm.run()
    }
}

#[test]
fn arithmetic() {
    let mut int = create_runtime(
        "main",
        "
        (def add (a b)
            (ret (+ a b)))
        (def sub (a b)
            (ret (- a b)))
        (def mul (a b)
            (ret (* a b)))
        (def div (a b)
            (ret (/ a b)))
        (def rem (a b)
            (ret (% a b)))
        ",
    );

    let add = int.call("add", &[1, 2]).unwrap();
    let sub = int.call("sub", &[1, 2]).unwrap();
    let mul = int.call("mul", &[1, 2]).unwrap();
    let div = int.call("div", &[1, 2]).unwrap();
    let rem = int.call("rem", &[1, 2]).unwrap();
    assert_eq!(LV2Value::from(3), add);
    assert_eq!(LV2Value::from(-1), sub);
    assert_eq!(LV2Value::from(2), mul);
    assert_eq!(LV2Value::from(0.5), div);
    assert_eq!(LV2Value::from(1), rem);
}

#[test]
fn recursive_faculty() {
    let mut int = create_runtime(
        "main",
        "
        (def fac (x) 
            (if (not (eq x 0))
                (ret (* x (fac (- x 1))))
                (ret 1)))
        ",
    );

    assert_eq!(LV2Value::from(1), int.call("fac", &[1]).unwrap());
    assert_eq!(LV2Value::from(2), int.call("fac", &[2]).unwrap());
    assert_eq!(LV2Value::from(6), int.call("fac", &[3]).unwrap());
    assert_eq!(LV2Value::from(5040), int.call("fac", &[7]).unwrap());
}

#[test]
fn looping() {
    let mut int = create_runtime(
        "loops",
        "
        (def looping (n)
            (let r 1)
            (let i 0)
            (loop
                (if (eq i n)
                    (break))
                (if (eq (% i 2) 0)
                    (do
                        (let i (+ i 1))
                        (continue)))
                (let r (* r i))
                (let i (+ i 1)))
            (ret r))
        ",
    );

    assert_eq!(LV2Value::from(1), int.call("looping", &[1]).unwrap());
    assert_eq!(LV2Value::from(1), int.call("looping", &[2]).unwrap());
    assert_eq!(LV2Value::from(3), int.call("looping", &[5]).unwrap());
}

#[test]
fn import_vice_versa() {
    let mut int = Interpreter::new();
    let a = create_module(
        "a",
        "
    (import b)
    (def main (x)
        (ret (b-inb)))
        ",
    )
    .unwrap();
    let b = create_module(
        "b",
        "
    (def inb ()
        (ret 1))
        ",
    )
    .unwrap();

    int.load(b).unwrap();
    int.load(a).unwrap();

    assert!(int.call("a-main", &[0]).is_ok());
}

#[test]
fn create_complex_types() {
    let mut int = Interpreter::new();
    let main = create_module(
        "main",
        r#"
    (def create-list (n)
        (ret (list n 2 "abc")))

    (def create-dict (key val)
        (ret (dict (key val) ("a" 1))))
        "#,
    )
    .unwrap();

    int.load_global(main).unwrap();

    let ls = int.call("create-list", &[true]).unwrap();

    assert_eq!(LV2Value::from(true), ls.get(&0.into()).unwrap());
    assert_eq!(LV2Value::from(2), ls.get(&1.into()).unwrap());
    assert_eq!(LV2Value::from("abc"), ls.get(&2.into()).unwrap());

    let dict = int.call("create-dict", &[1, 2]).unwrap();

    assert_eq!(LV2Value::from(2), dict.get(&1.into()).unwrap());
    assert_eq!(LV2Value::from(1), dict.get(&"a".into()).unwrap());
}

#[test]
fn foreach() {
    let mut int = Interpreter::new();
    let main = create_module(
        "main",
        r#"
    (def sum (n)
        (let res 0)
        (foreach ((range 1 (+ n 1)) i)
            (let res (+ res i)))
        (ret res))
        "#,
    )
    .unwrap();

    int.load_global(main).unwrap();

    assert_eq!(LV2Value::from(6), int.call("sum", &[3]).unwrap());
    assert_eq!(LV2Value::from(10), int.call("sum", &[4]).unwrap());
    assert_eq!(LV2Value::from(15), int.call("sum", &[5]).unwrap());
}

#[test]
fn converting() {
    let mut int = Interpreter::new();
    let main = create_module(
        "main",
        r#"
    (def as-bool (n)
        (ret (bool n)))
    (def as-float (n)
        (ret (float n)))
    (def as-int (n)
        (ret (int n)))
    (def as-str (n)
        (ret (str n)))
    "#,
    )
    .unwrap();

    int.load_global(main).unwrap();

    assert_eq!(LV2Value::from(true), int.call("as-bool", &[3]).unwrap());
    assert_eq!(LV2Value::from(5.), int.call("as-float", &[5]).unwrap());
    assert_eq!(LV2Value::from(1), int.call("as-int", &[true]).unwrap());
    assert_eq!(LV2Value::from("4"), int.call("as-str", &[4]).unwrap());
}
