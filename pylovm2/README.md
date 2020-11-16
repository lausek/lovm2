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
main_hir = module.entry().code()
main_hir.assign('n', 2)
# call the module local function `add` with the value of `n`
main_hir.call('print', 'got result:', Expr.call('add', Expr.var('n'), 1), '\n')
main_hir.call('print', 'got result from pyfn:', Expr.call('pyadd', Expr.var('n'), 1), '\n')

# add new entry with arguments `a` and `b`
# note: no direct usage of `.code()`
add_hir = module.add('add')
add_hir.args(['a', 'b'])
# use block builder from now on
add_hir = add_hir.code()
add_hir.ret(Expr.add(Expr.var('a'), Expr.var('b')))

# add a python function to the module
module.add('pyadd').pyfn(lambda a, b: a.to_py() + b.to_py())

# build the module and print it
module = module.build()
print(module)

# create vm, load and run module
vm = Vm.with_std()
vm.load(module)
vm.run()
```

## Building and Publishing

**NOTE:** manylinux wheels are required for distribution

``` bash
sudo docker build -t pylovm2-build .
sudo docker run -it -v $(pwd):/io pylovm2-build

# ... or use `maturin build`
$ maturin publish
```
