## Build buck2 with Cargo

From buck2 project root, run the following to build buck2 with cargo

```sh
cargo install --path=app/buck2 --root=/tmp
export BUCK2="/tmp/bin/buck2"
```

## Run `buck2 init --git`

Run `buck2 init` to initialize the prelude directory.

Now all targets aside from OCaml related ones are ready to be built.

## Support for building the example OCaml targets

The information in this section is (at this time) Linux and macOS specific.

The commands in `setup.sh` assume an activated [opam](https://opam.ocaml.org/) installation. Their effect is to create symlinks in the 'third-party/opam' directory. These symlinks support building the example OCaml targets. If any of the symlinks are found to already exist, they will not be overwritten.

## Sample commands

**_NOTE:_** These commands are currently only supported on Linux and macOS.

```sh
$BUCK2 build //ocaml/...
$BUCK2 run //python/hello_world:main
```
