all: clean build_asan build_ubsan

clean:
	rm *.exe

build_asan:
	 clang -Wall -Wextra -fsanitize=address -g san_test.c ../benchmarker_internal.c -o benchmark_test_asan.exe

build_ubsan:
	 clang -Wall -Wextra -fsanitize=undefined -g san_test.c ../benchmarker_internal.c -o benchmark_test_ubsan.exe

run: run_asan run_ubsan

run_asan: build_asan
	echo "Running with AddressSanitizer (ASan)"
	./benchmark_test_asan.exe

run_ubsan: build_ubsan
	echo "Running with UndefinedBehaviorSanitizer (UBSan)"
	./benchmark_test_ubsan.exe

.PHONY: all clean build_asan build_ubsan run run_asan run_ubsan
