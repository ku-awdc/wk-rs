# wk-rs

## Build

You must have R available in the `PATH`.

### Windows

You must have Rtools installed. Set the `target` and the `linker` in Cargo,
for example by providing a `.cargo\config.toml` in the root of this crate, with

```toml
[build]
# # target-dir = "target"
target = "x86_64-pc-windows-gnu"

[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32.static.posix-gcc.exe"
```

## Developers

IMPORTANT: Mac users may want to delete the contents of `.cargo/config.toml`,
as this specifies things necessary for building on Windows specifically.

R must be available in the `PATH` for the build to run,
and on Windows you need to install Rtools as well.

### Generate Bindings

This repo uses `git submodule`. If you wish to upgrade the bindings,
you must update the attached modules, e.g. `wk`, `cpp11`, etc.

```shell
git submodule init
git submodule update
```

To regenerate bindings, run `cargo build --features export_bindings`. Note,
that edits to the bindings must be carried over.

See also [notes](notes.md) for more information.
