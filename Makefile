build:
	cargo build --release

clean:
	cmd /C rd /s /q target

.PHONY: make-was-easier-to-type clean
