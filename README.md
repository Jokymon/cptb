cptb
====

Abstract
--------

An experiment to create a simple C++ toolbelt command line tool that should
help with creating and building CMake/C++ projects and managing different
combinations of toolchains and CMake versions. The inspiration are the
'cargo' and 'rustup' tools from the Rust world.

Objectives
----------

### Goals

- a single, standalone executable without dependencies
- allow for quick and easy start of a C++ project
- minimal manual setup required

### Non-Goals

- a complete equivalent of rust/cargo for C++
- provide dozens of options to support any possible CI/unittesting/coverage/...
  combination for setup and configuration right at creation time
- C++ package manager that might replace Conan/Hunter/vcpkg/...

### Some decisions

- Rust was used as implementation language because it was very easy to build
  a standalone executable without external dependencies right out of the box
  and because a lot of very helpful libraries are easily available in Rust,
  such as clap, handlebars, rust-embed, ... Additionally I really just wanted
  to create something productive in Rust ;-)

Requirements and building
-------------------------

To build the cptb executable you just need a decent Rust compiler with which
you run

```
cargo build
```

Usage
-----

Once you have built the cptb executable, unfortunately at the moment you still
need to setup some manual configuration with your toolchains. After that however,
building your project should be really easy.

### Manual configuration

One of the strengths of cptb is the simplified handling of different toolchains
which should be particularly helpful on Windows. To support this simplified
toolchain handling, cptb needs a few configuration files.

The configuration is stored in `%HOME_DIRECTORY%/.cptb`. This directory requires
two JSON-files that contain the CMake and toolchain descriptions. The file
`kits.json` can contain multiple toolchain descriptions as follows:

```json
{
    "compilers": {
        "mingw-8-1": {
            "name": "MinGW-w64 8.1.0 SEH",
            "path": "c:/mingw64/x86_64-8.1.0-release-win32-seh-rt_v6-rev0/bin",
            "cmake_generator": "MinGW Makefiles"
        }
    },
    "cmake": {
        "cmake-3-17": {
            "name": "CMake 3.17",
            "path": "C:/Program Files (x86)/CMake/bin"
        }
    },
    "kits": {
        "cmake-3-17_mingw-8-1": {
            "name": "MinGW 8.1.0; CMake 3.17",
            "compiler": "mingw-8-1",
            "cmake": "cmake-3-17"
        }
    }
}
```

The `kits.json` file contains arbitrary key names that describe individual kits.
One such kit is then refered to from the `settings.json` file as the
`default_kit`, which is used by cptb for building. The `"toolchain"` key is a
path to the compiler executable. The `"cmake"` key describes the CMake version
to be used for building. The `"path"` sub-key again describes the path to the
cmake executable. The `"generator"` key is an optional key for giving the value
to the `-G` parameter of CMake.

```json
{
    "default_kit": "cmake-3-17_mingw-8-1"
}
```

### Creating a new project

Switch to a directory in which you want to create your new C++ project and run

```
cptb new my_project
```

This will create a new directory called `my_project` which contains a simple
hello world application written in C++ which could be built with CMake and
a C++ compiler.

### Building a project

When you have setup your toolchains properly as described above and have
created a project, you can now build it. Simply change into the directory
of the new project and run `cptb build`:

```
cd my_project
cptb build
```

The initial hello world project should build right out of the box and you
should now have your executable inside the `build/` folder of your project.

### Getting a shell in the build environment

When running `cptb build` this will temporarily update the `PATH` variable
and potentially other environment variables to perform the build itself.
However after running the `build` subcommand, these changes are no longer
visible.

To start a new shell with all environment variables set according to the
build environment, simply run

```
cptb buildenv
```

Now you can run `cmake`, `make` or any other build tools available from your
toolchain. In the new shell started by `cptb` you will see your original
prompt prepended with a `(cptb build)` to indicate that you are in a special
environment.

Leaving the build environment is as easy as

```
exit
```


License
-------

[MIT](LICENSE.txt)
