# How does the Build Process work?

Blackheap consists of 3 different programs written in C, Typescript and Rust respectively. A more detailed explanation of the architecture and its reasoning behind it can be found under [Architecture](../Architecture.md). In this section blackheap's build process is explained. To make this as language independent as possible, it is orchestrated using a Makefile.

## Step 1: Building the Benchmarker
First, the benchmarker written in C is compiled. gcc is used as compiler; the compilation process is managed by a second Makefile. At the end, the built binary gets moved into the rust source code directory.

## Step 2: Building the Web-Frontend

The next step is to build the web-frontend. This is a React frontend based on [CRA](https://create-react-app.dev/). [yarn](https://yarnpkg.com/) is used as a package manager.

First the project gets installed via yarn. Here all external node-modules dependencies are resolved. After that a minimized, production level optimized single HTML file gets built. At the end, this HTML file also gets moved into the rust directory.

## Step 3: Building the Main Program

The last thing to be built is the Rust binary. Here the previously built C and TS programs are embedded as binary BLOBs within the main program. Hence, the Rust program can ship the other software optimized for the current architecture at runtime and blackheap can be deployed as a single binary.
