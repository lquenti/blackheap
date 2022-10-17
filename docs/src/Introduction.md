# Introduction

Analyzing I/O requets is very difficult. It requires a lot of specialized knowledge. One has to take into account the file system, server topology, different traffic sources and access patterns among other things. This is not feasible for the average HPC user, as they neither have the permissions nor the time or knowledge for this type of sophisticated analysis.

Furthermore, any formal, deterministic analysis is impossible, since the access time is dependent on too many parameters. It is not practical to try to model the relevance of disk seek times or the linux block layer schedulers.

[*Blackheap*](https://github.com/lquenti/blackheap) approaches this problem with a blackbox methology. It generates a predictive performance model based on the access time alone.

## Goals

We designed Blackheap with the following goals in mind:

- Single binary without external dependencies
- No required configuration
- Self-contained, automatic end-to-end workflow
- Setup agnostic classification (remote or local storage)
- Distribution agnostic: Any glibc-based Linux distribution will suffice
- No root required
- Easy to build: only non-userspace dependencies are `glibc`, `gcc` and `make`

## High Level Workflow

The internal workflow can be summarized as follows:

1. At first, we do a lot of benchmarks with different access patterns. Those access patterns are used to isolate different characteristics like hitting the page cache.

2. After that, we analyze each benchmark run. By using a kernel density estimation, we find the most significant cluster. This cluster provides an upper bound for the expected time.

3. Lastly, we take all those upper bounds and create a predictive model via linear regression.

## Further Ressources

Blackheap is based on [Julian Kunkels](https://hps.vi4io.org/about/people/julian_kunkel) 2015 paper "[Identifying Relevant Factors in the I/O-Path
using Statistical Methods](https://hps.vi4io.org/_media/research/publications/2015/dlirfitiusmk15-identifying_relevant_factors_in_the_i_o_path_using_statistical_methods.pdf)" ([BibTeX](https://hps.vi4io.org/bibtex?bibtex_source=publications&bibtex_key=IRFITIUSMK15)).
