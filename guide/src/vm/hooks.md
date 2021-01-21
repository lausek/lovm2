# Hooks

## Load Hook

`vm.set_load_hook(callback)`

The load hook is a special function bound to the `Vm` that will be consulted first whenever a module should be loaded into the `Vm`. It returns an `Option` containing the correct `Module` structure if the hook was able to resolve the requested name on its own.

## Import Hook

`vm.set_import_hook(callback)`

The import hook handles naming of functions being imported into the scope. As such it can also be used to adjust the naming scheme of the `lovm2` standard library.

The function signature expects the callback to return an `Option<String>` where `Some("name")` will proceed importing with a new identifier. Importing a function can be avoided by returning `None`.
