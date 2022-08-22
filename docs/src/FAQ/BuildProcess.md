# How does the Build Process work?

- The architecture, and its reasoning, are described under [Architecture](../Architecture.md).

- In order to stay language agnostic, we use a simple `Makefile` to orchestrate the different build systems.

## The Benchmarker
- At first, we build the benchmarker. Its build process is also managed by a seperate Makefile.


## The Web-Frontend
- Next, we build the web frontend
- Its build process is managed by yarn and CRA
- CRA is ejected and extended with (PLUGIN A UND PLUGIN B) in order to create a single HTML file

## The Modeller / Main Program
- This build process is done via cargo
- Beforehand, we move the benchmarker and web frontend into the rust src folder
- this way, we can embed both files in the binary, making it a single standalone workflow
