# Why do we require depend on glibc?

We depend on [the GNU C library `glibc`](https://www.gnu.org/software/libc/) for multiple reasons:

- It provides the [most accurate clock](https://stackoverflow.com/questions/12392278/measure-time-in-linux-time-vs-clock-vs-getrusage-vs-clock-gettime-vs-gettimeof/12480485#12480485) via [`CLOCK_MONOTONIC`](https://linux.die.net/man/2/clock_gettime).
- It is required for [`O_DIRECT`](https://man7.org/linux/man-pages/man2/open.2.html) file access
- It provides [`argp`](https://www.gnu.org/software/libc/manual/html_node/Argp.html) as a better argument parsing alternative to the POSIX [`getopt`](https://en.wikipedia.org/wiki/Getopt).
