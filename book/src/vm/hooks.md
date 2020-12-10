# Hooks

## Load Hook

`vm.set_load_hook(callback)`

The load hook is a special function bound to the `Vm` that will be consulted first whenever a module should be loaded into the `Context`.

If the hook is able to resolve the requested name to a module, it can return 

## Import Hook

`vm.set_import_hook(callback)`

The import hook handles the naming of functions being imported into the scope.
