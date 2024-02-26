# Blackheap

# BIG RECODE, `old-v0` is the old branch

This tool creates a performance model of your I/O speed and further allows to predict future preformance.

## Note

There is a **big recode** right now. But how does one eat an elephant...

## How to get it running

### Locally, normal device

Normally
```
cargo build --release
```
should suffice

### [SCC cluster](https://gwdg.de/hpc/systems/scc/)
- Use `rustup`, not modules
- Get a up to date rust compiler via `rustup update`

- `cc`, which is mapped to the default `gcc`, is too old.
  - Load a newer gcc via `module load gcc/11.4.0`
  - Tell rust to use that one via `CC=$(which gcc) cargo build --release`


### [Emmy HLRN cluster](https://gwdg.de/hpc/systems/emmy/)
- Use `rustup`, not modules
- Get a up to date rust compiler via `rustup update`

- get the newest `gcc` as module as well
  - Tell rust to use that one via `CC=$(which gcc) cargo build --release`
