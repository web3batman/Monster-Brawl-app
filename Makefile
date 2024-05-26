.PHONY: clean fmt run test help

clean:
	cargo clean

build:
	cargo build

fmt:
	cargo fmt --all -- --check

run:
	cargo watch -c -q -w src/ -x run

test:
	cargo watch -c -q -x 'test -- --nocapture'

help:
	@echo "build - compile the project"
	@echo "clean - remove the target directory"
	@echo "fmt - format the code"
	@echo "run - run the project in watch mode"
	@echo "test - excute all unit tests"
	@echo "help - print this help message"
