IMAGE_NAME = iot/yanbing-edge
REPOSITORY = docker.diweiyunlian.cn:1443/${IMAGE_NAME}
TAG = latest

build-image:
	docker build -t $(REPOSITORY):$(TAG) . --progress=plain --no-cache

push-image:
	docker push $(REPOSITORY):$(TAG)

run-container: build-image
	docker run -d --name yanbing-edge -p 8000:8000 $(REPOSITORY):$(TAG)

stop-container:
	docker stop yanbing-edge

remove-container: stop-container
	docker rm yanbing-edge

remove-image: remove-container
	docker rmi $(REPOSITORY):$(TAG)

check:
	cargo check
fix:
	cargo fix --allow-dirty
build-release:
	cargo build --release