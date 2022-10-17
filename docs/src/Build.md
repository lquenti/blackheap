# How to Build Blackheap

## Prerequisites
- Have a `glibc` based linux distribution (no alpine support)
- Required: `gcc`
- Optionally: `make`

## Setting up userspace tooling
In order to compile it locally, we need the followind userspace tooling. This can be skipped if the tools are installed globally.

### Node and npm
The easiest way to install node is via the [Node version manager](https://github.com/nvm-sh/nvm). After installing nvm and reloading your `shellrc` node can be installed via

```
nvm install node --lts
nvm use node --lts
```

This automatically installs npm as well.

### yarn
If node and npm are already installed, yarn can be globally installed via 
```
npm i -g yarn
```

### rustc + cargo
The easiest way to install Rust is via [rustup](https://rustup.rs/). It should automatically install the newest stable version.

## Building Blackheap
If everything worked, blackheap should be buildable via `make`.
