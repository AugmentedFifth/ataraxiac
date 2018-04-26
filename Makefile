.PHONY: clippy debug release native

clippy:
	cargo clippy -- -W useless-attribute &>clippy.log && perl -0777 -pi -e 's/warning: ([^\/]|\n)+?\/ataraxia\.rs:(.|\n)+?(.|\n)= help:.+//g' clippy.log

debug:
	cargo build

release:
	cargo rustc --release -- -C target-feature=+crt-static && strip ./target/release/ataraxiac

native:
	cargo rustc --release -- -C target-cpu=native && strip ./target/release/ataraxiac
