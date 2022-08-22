# Why do we require depend on glibc?

We depend on glibc for multiple reasons:

- It provides the most accurate clock via [`CLOCK_MONOTONIC`](https://linux.die.net/man/2/clock_gettime) [(see)](https://stackoverflow.com/questions/12392278/measure-time-in-linux-time-vs-clock-vs-getrusage-vs-clock-gettime-vs-gettimeof/12480485#12480485)
- It is required for `O_DIRECT` file access
- It makes argument parsing more elegant with argp instead of getopt
