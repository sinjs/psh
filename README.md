# psh - the pene shell

welcome to the pene shell, a simple and fast shell made in rust.

it's...

- ~~**p**owerful~~ not quite yet :)
- **p**ortable and made for every operating system which supports the basic standard library
- based on **p**eneos standard library

### **d**ownloading

you are able to download the latest binary release from the [releases section](https://github.com/sinjs/psh/releases) on github, they are
automatically compiled after every commit, so if you want a non-tagged release, like a binary from
a specific commit, you are able to search for the commit and check the github actions output

### **b**uilding

#### requirements

- rustc `edition=2021`
- cargo

> **note**
>
> if you want to build for pene, you will need to clone the pene github repository
> and set up a cross compiler (toolchain directory) and use that compiler:
> target: `x86_64-pc-pene-gnu`

#### compilation

this application can be compiled just like any other rust application, **cargo is required**
incase you are unfamiliar, here are some commands just for reference, however i recommend reading
the rust book / cargo book for more information:

```sh
# compiling and running
cargo run              # dev, made for debugging
cargo run --release    # release, optimized

# building
cargo build            # dev, made for debugging
cargo build --release  # release, optimized

# testing
cargo test
```
