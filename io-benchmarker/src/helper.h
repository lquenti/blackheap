#ifndef IO_BENCHMARK_HELPER_H
#define IO_BENCHMARK_HELPER_H

#include<stdbool.h>
#include<stddef.h>
#include<sys/stat.h>
#include<unistd.h>

#define MEMINFO "/proc/meminfo"

/* See: https://unix.stackexchange.com/q/17936 */
#define DROP_PAGE_CACHE "/proc/sys/vm/drop_caches"

/* See: https://stackoverflow.com/a/70370002/9958281 */
const unsigned long MAX_IO_SIZE;

/** Helper function to "handle" malloc failure.
 *
 * In this case, handling means logging failure and killing the whole program.
 * This is fine because we have a job-based architecture and this job obviously failed.
 */
void *malloc_or_die(size_t size);


/** Helper function to "handle" open failure.
 *
 * In this case, handling means logging failure and killing the whole program.
 * This is fine because we have a job-based architecture and this job obviously failed.
 */
int open_or_die(const char *pathname, int flags, mode_t mode);


/** Helper function to "handle" read failure.
 *
 * In this case, handling means logging failure and killing the whole program.
 * This is fine because we have a job-based architecture and this job obviously failed.
 *
 * Also: the maximum size which is checked aginst is defined in Linux man 2 read
 */
ssize_t read_or_die(int fd, void *buf, size_t count);


/** Helper function to "handle" write failure.
 *
 * In this case, handling means logging failure and killing the whole program.
 * This is fine because we have a job-based architecture and this job obviously failed.
 *
 * Also: the maximum size which is checked aginst is defined in Linux man 2 write
 */
ssize_t write_or_die(int fd, void *buf, size_t count);

/** Helper function to "handle" close failure.
 *
 * In this case, handling means logging failure and killing the whole program.
 * This is fine because we have a job-based architecture and this job obviously failed.
 */
int close_or_die(int fd);

/** Helper function to "handle" lseek failure.
 *
 * In this case, handling means logging failure and killing the whole program.
 * This is fine because we have a job-based architecture and this job obviously failed.
 */
off_t lseek_or_die(int fd, off_t offset, int whence);

/** Helper function to "handle" fsync failure.
 *
 * In this case, handling means logging failure and killing the whole program.
 * This is fine because we have a job-based architecture and this job obviously failed.
 */
int fsync_or_die(int fd);

/** Helper function to "handle" fstat failure.
 *
 * In this case, handling means logging failure and killing the whole program.
 * This is fine because we have a job-based architecture and this job obviously failed.
 */
int fstat_or_die(int fd, struct stat *st);


/** Checks for io-failure outside of benchmark.
 *
 * This is done in order to minimize branching in the actual io-read, thus
 * giving the best possible benchmark data.
 */
void io_op_worked_or_die(int res, bool is_read_operation);


void remove_or_die(const char *pathname);
void strn_to_lower(char *str, size_t n);
long parse_from_meminfo(char *key);
size_t get_available_mem();
void allocate_memory_until(size_t space_left_in_kib);
/* See: https://unix.stackexchange.com/q/17936 */
void drop_page_cache();

#endif //IO_BENCHMARK_HELPER_H
