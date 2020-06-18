RUST_CHANNEL="nightly"

.PHONY: build run test

build:
	docker build --build-arg CHANNEL="${RUST_CHANNEL}" -t diegorubin/rust-builder .
run:
	docker run -v $$PWD/test:/volume  -w /volume -it diegorubin/rust-builder /bin/bash

clean-docker:
	docker images diegorubin/rust-builder -q | xargs -r docker rmi -f
clean-builds:
	sudo find . -mindepth 3 -maxdepth 3 -name target -exec rm -rf {} \;
clean: clean-docker clean-builds

