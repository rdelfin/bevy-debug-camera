TAG=1.67.0
IMAGE_BASE_NAME=rdelfin/rust_ci
IMAGE_NAME=$(IMAGE_BASE_NAME):$(TAG)

.PHONY: build
build:
	docker build rdelfin/rust_ci:1.67.0

.PHONY: publish
publish: build
	docker push rdelfin/rust_ci:1.67.0
