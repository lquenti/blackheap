# Architecture

We have a very specific and unusual architecture:

- Our benchmarker is written in C
- Our frontend is written in React (Typescript)
- The modelling logic and main program is written in Rust

- Note that those components are actually 3 different programs, not just libraries.
- But its still a fully automated process; no user input is required whatsoever

## How
- The rust program generates the run parameters
- Creates a new process for each run, the run results get saved to disk in case of WHAT
- Afterwards, the rust program processes the same results as well, and saves the generated models.
- It ALSO saves a single HTML file for local visual analysis.
  - Note that this does not require a local web server. See the build section for more.

## Why?
### Why Rust
- Initial prototype was actually written in Python
- We learned that we can't expect python to be installed everywhere
- Although solutions like [pyenv](https://github.com/pyenv/pyenv) exist, this is not a simple solution
- Rust offered a high level library without any dependencies and a great, although not yet mature, library ecosystem with cargo.
  - Most rust libs don't have 3rd party dependencies as well
  - Also pretty easy to build and link external libraries statically via `build.rs`

### Why C
- Yes yes, rust is performant I know
- But we wanted to be as near to the kernel API and bare metal as possible
- Also some calls like `lseek` aren't even available in the rust stdlib

### Why React / TS
- Building Plotting libraries is hard.
- Rust state of the art plotting libraries are either
  - still very small
  - or have 3rd party dependencies
- Thus I decided to use the great web tooling for a plotting UI
- React because it is the most used frontend framework; thus most likelyhood to be maintainable in the future
- It also allows to share the results on the web, which is nice
