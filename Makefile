.PHONY: all run watch release

.SILENT:

all: run

run:
	cargo run

watch:
	systemfd --no-pid -s http::3000 -- cargo watch -x run

release:
	cargo build --release
