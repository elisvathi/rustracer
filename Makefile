rustracer: build run
release: build-release run-release
build:
	cargo build
build-release:
	cargo build --release
run-release:
	cd target/release && ./test_image.sh
run:
	./test_image.sh
