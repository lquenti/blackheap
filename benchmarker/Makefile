CC = gcc
CFLAGS = -std=c11 -O2 -Wall -Wextra -Wpedantic

blackheap_benchmarker: blackheap_benchmarker.c
	$(CC) $(CFLAGS) -o $@ $<

clean:
	rm -f blackheap_benchmarker

.PHONY: clean
