all: clean make-build-folder build show-help

clean:
	rm -rf build

make-build-folder: clean
	mkdir build

build: make-build-folder
	gcc -Wall -Wextra -Wpedantic -std=gnu11 -O2 -D_GNU_SOURCE -o ./build/io-benchmark.exe ./src/*.c -lrt

show-help: build
	./build/io-benchmark.exe --help

example: build
	./build/io-benchmark.exe --read --file=/dev/shm/example_benchmark.dat --mem-pattern=seq --file-pattern=seq --repeats=5 --mem-buf=5242880 --file-buf=67108864 --access-size=1048576

special-example: build
	./build/io-benchmark.exe --read --file=/dev/zero --mem-pattern=seq --file-pattern=rnd --repeats=5 --mem-buf=5242880 --file-buf=67108864 --access-size=1048576

.PHONY: all clean make-build-folder build show-help
