# Architecture

We have a very specific and unusual architecture:

- Our benchmarker is written in C
- Our frontend is written in React (Typescript)
- The modelling logic and main program is written in Rust

Note that those components are actually 3 different standalone programs, not just libraries linked together. Although those programs are fully functional on their own, blackheap still provides a fully automated process; no user input is required. Moreover, blackheap is shipped as a single binary, see [the build process for more](./FAQ/BuildProcess.md).

## How does this work?

At first, the rust program generates the proper parameters for each benchmark run. It then sequentially spawns a new process for each run. After each run the results get saved to disk. This allows reproducibility as well as reanalyzing old benchmarks once new models are developed.

After all benchmarks were run, the rust program processes the same results as described above. It then saves the generated models.

It lastly also saves a single HTML file based frontend for local analysis, see [the local workflow](./SingleNode.md) for more information. Note that this tool can be used without a local web server, since all assets are transpiled into one HTML file. See [the build section](./FAQ/BuildProcess.md) for more.

## Why was Rust chosen?
The intial prototype was written in Python3. But, especially in lightweight HPC environments running old RHEL-based distributions, Python3 often is not installed.

Although solutions like [pyenv](https://github.com/pyenv/pyenv) exist, bootstrapping Python is not always trivial. Thus we wanted to create a dependency-less binary while keeping most advantages provided by the Python ecosystem.

This is why we used Rust. Rust offers a huge high level ecosystem with `cargo`. Rust has zero runtime dependencies and bootstrapping Rust via [`rustup`](https://rustup.rs/) requires no preinstalled libraries.

Furthermore, most Rust libraries don't have any 3rd party non-cargo dependencies as well.

### Why was C chosen?

Despire Rust being a very performant programming language with very little performance overhead, it didn't suffice for our benchmarks. In order to get the best possible accuracy, we need to stay as near to the kernel API as possible. Also, some calls like `lseek` aren't even available in the Rust stdlib.

### Why was React / TS chosen?

Rust plotting libraries are either very immature or have hard dependencies on 3rd-Party libraries. Since it was impractical to implement a plotting library from scratch, we chose to take advantage of the enormous Web ecosystem.

We chose React since it is the most used frontend framework, which maximizes the likelyhood of being easy to maintain in the future. We furthermore chose Plotly as a plotting library since it has enough commercial support to secure its future maintenance.
