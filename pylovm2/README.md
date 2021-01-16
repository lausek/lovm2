# pylovm2

python bindings for [lovm2](https://github.com/lausek/lovm2).

``` bash
pip3 install pylovm2
```

## Example

``` python
from pylovm2 import Expr, ModuleBuilder, Vm

# initialize a new module
module = ModuleBuilder()

# add the main entry point
main_hir = module.entry()
main_hir.assign('n', 2)
# call the module local function `add` with the value of `n`
main_hir.call('print', 'got result:', Expr.call('add', Expr.var('n'), 1), '\n')
main_hir.call('print', 'got result from pyfn:', Expr.call('pyadd', Expr.var('n'), 1), '\n')

# add new entry with arguments `a` and `b`
add_hir = module.add('add', ['a', 'b'])
add_hir.ret(Expr.add(Expr.var('a'), Expr.var('b')))

# add a python function to the module
module.add_pyfn('pyadd', lambda a, b: a.to_py() + b.to_py())

# build the module and print it
module = module.build()
print(module)

# create vm, load and run module
vm = Vm.with_std()
vm.add_module(module)
vm.run()
```

## Building and Publishing

**NOTE:** manylinux wheels are required for distribution

``` bash
docker build -t pylovm2-build .
docker run -it -v $(pwd):/io pylovm2-build

# ... or use `maturin build`
$ maturin publish
```

### Problems with SSL

manylinux wheels are not allowed to link to TLS libraries dynamically. Building without SSL support can be done like this:

```
maturin build --cargo-extra-args="--no-default-features --features lovm2/stdlib,lovm2/stdlib-net"
```
