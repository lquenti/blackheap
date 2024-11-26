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

### [Emmy HLRN cluster](https://gwdg.de/hpc/systems/emmy/)
- Use `rustup`, not modules
- Get a up to date rust compiler via `rustup update`


## Why the Name Blackheap?

In blackheap, we use a __blackbox__ methodology for classifying I/O requests. So "blackbox" would be a obvious name choice. Of course, this would be a horrible name: Not only for SEO reasons but also for general name collisions.

In Rust, the [`Box<>`](https://doc.rust-lang.org/book/ch15-01-box.html) is the simplest data type to provide a smart pointer. Basically, if you put something in a `Box`, you store it on the Heap while the only thing put on the stack is the corresponding pointer.

Thus, substituting `Box` in blackheap for being a pointer on the heap, we have __blackheap__!
