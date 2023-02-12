TAG=1.67.0
IMAGE_BASE_NAME=rdelfin/rust_ci
IMAGE_NAME=$(IMAGE_BASE_NAME):$(TAG)

.PHONY: build
build:
	docker build -t $(IMAGE_NAME) .

.PHONY: publish
publish: build
	docker push $(IMAGE_NAME)
