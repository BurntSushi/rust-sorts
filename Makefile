RUST_CFG ?= --cfg sorted --cfg same --cfg micro --cfg small --cfg medium --cfg large
RUST_PATH ?= $(HOME)/.rust/lib/x86_64-unknown-linux-gnu

compile:
	rustc -L $(RUST_PATH) ./src/lib.rs

install:
	cargo-lite install

tags:
	ctags --recurse --options=ctags.rust --languages=Rust

docs:
	rm -rf doc
	rustdoc src/lib.rs
	# WTF is rustdoc doing?
	chmod 755 doc
	in-dir doc fix-perms
	rscp ./doc/* gopher:~/www/burntsushi.net/rustdoc/

test: sort-test
	RUST_TEST_TASKS=1 RUST_LOG=quickcheck,sorts ./sort-test

sort-test: src/lib.rs src/test.rs src/bench.rs
	rustc -L $(RUST_PATH) -O --test src/lib.rs -o sort-test

bench: bench-test
	RUST_TEST_TASKS=1 RUST_LOG=quickcheck,sorts ./sort-test --bench --save-metrics=bench.json

bench-test: src/lib.rs src/test.rs src/bench.rs
	rustc -L $(RUST_PATH) -O --test $(RUST_CFG) src/lib.rs -o sort-test

test-clean:
	rm -rf ./sort-test

clean: test-clean
	rm -f *.rlib

push:
	git push origin master
	git push github master

