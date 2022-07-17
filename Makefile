# TODO: Logging

all: clean prepare build-rust

# TODO: Clean up rest of the repos
clean:
	rm -rf ./blackheap-modeller/assets

prepare:
	mkdir ./blackheap-modeller/assets

build-rust: build-react build-c
	cd blackheap-modeller && \
	cargo build --release && \
	cd .. && \
	cp ./blackheap-modeller/target/release/blackheap .

build-react:
	cd blackheap-frontend && \
	yarn && \
	yarn build && \
	cd .. && \
	cp ./blackheap-frontend/build/index.html ./blackheap-modeller/assets

build-c:
	cd blackheap-benchmark && \
	make build && \
	cd .. && \
	cp ./blackheap-benchmark/build/blackheap-benchmark.exe ./blackheap-modeller/assets

.phony: all clean build-rust build-react build-c
