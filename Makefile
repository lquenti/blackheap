all: clean prepare build-rust

run: build-rust
	./blackheap

clean:
	rm -f ./blackheap-modeller/assets/blackheap-benchmark.exe
	rm -f ./blackheap-modeller/assets/index.html
	rm -f blackheap
	rm -rf ./blackheap-benchmark/build
	rm -rf ./blackheap-frontend/{build,node_modules}


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
