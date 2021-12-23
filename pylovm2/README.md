# pylovm2

Create your own programming language in Python on top of [lovm2](https://github.com/lausek/lovm2).

``` bash
pip3 install pylovm2
```

## Example

``` python
from pylovm2 import LV2Expr, LV2ModuleBuilder, LV2Variable, LV2Vm

# initialize a new module
module = LV2ModuleBuilder()

# declare the variables we want to use
n, a, b = LV2Variable("n"), LV2Variable("a"), LV2Variable("b")

# add the main entry point
main_hir = module.entry()
main_hir.assign("n", 2)
# call the module local function `add` with the value of `n`
main_hir.call("print", "got result:", LV2Expr.call("add", n, 1), "\n")
main_hir.call("print", "got result from pyfn:", LV2Expr.call("pyadd", n, 1), "\n")

# add new entry with arguments `a` and `b`
add_hir = module.add("add", [a, b])
add_hir.ret(LV2Expr(a).add(b))

# add a python function to the module
module.add_pyfn("pyadd", lambda a, b: a.to_py() + b.to_py())

# build the module and print it
module = module.build()
print(module)

# create vm, load and run module
vm = Vm.with_std()
vm.add_main_module(module)
vm.run()
```

## Building and Publishing

**NOTE:** manylinux wheels are required for distribution

``` bash
# just build the python package in release mode
# compiled wheels will be available inside ./target/wheels
./build.sh 

# attempt publishing package on pypi.org as well
./build.sh release
```

### Problems with SSL

manylinux wheels are not allowed to link to TLS libraries dynamically. Building without SSL support can be done like this:

```
maturin build --cargo-extra-args="--no-default-features --features lovm2/stdlib,lovm2/stdlib-net"
```
