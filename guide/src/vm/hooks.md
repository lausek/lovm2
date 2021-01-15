# Hooks

## Load Hook

`vm.set_load_hook(callback)`

The load hook is a special function bound to the `Vm` that will be consulted first whenever a module should be loaded into the `Context`. It returns an `Option` containing the module if the hook is able to resolve the requested name.

## Import Hook

`vm.set_import_hook(callback)`

The import hook handles the naming of functions being imported into the scope. As such it can also be used to adjust the naming scheme of the `lovm2` standard library.
