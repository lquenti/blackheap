# How to Build

## Prerequisites
- Have a `glibc` based linux distribution (no alpine support)
- Have `gcc` installed and in PATH somehow
- `make` makes things a lot easier
  - If not possible, just run the commands yourself, the Makefile is pretty readable

## Setting up userspace tooling
- We need `node+npm`, `yarn` and `rustc+cargo`
- It only has to be locally installed into userspace iff they aren't globally installed yet

### Node and npm
- easiest way is via the [Node version manager](https://github.com/nvm-sh/nvm)
- after installing nvm and reloading your shellrc
```
nvm install node --lts
nvm use node --lts
```
- this automatically installs npm as well
- Try out afterwards whether it worked via `node --version`

### yarn
- If node and npm are already installed, yarn can be globally installed via
```
npm i -g yarn
```

### rustc + cargo
- easiest way is to manage the installations via [rustup](https://rustup.rs/).
- After running the script and reloading your shell it should work

## Building Blackheap
- If everything worked, it should be buildable via `make`.
- If make is not accessible, just run the `build-*` targets specified in the `Makefile` in the correct order.
- check whether it worked via `./blackheap --help`
