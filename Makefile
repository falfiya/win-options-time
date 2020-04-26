build:
	cargo build --release

clean:
	cmd /C rd /s /q target

.PHONY: build clean
