all: clean make-build-folder build

clean:
	rm -rf build

make-build-folder: clean
	mkdir build

build: make-build-folder
	gcc -Wall -Wextra -std=gnu11 -O2 -fPIC -shared -o ./build/preloadee.so preloadee.c -ldl

.PHONY: all clean make-build-folder build
