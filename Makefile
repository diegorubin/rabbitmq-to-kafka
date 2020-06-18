.PHONY: build run test

build:
	cargo build --release
	sudo chown -R "${USER}" .
	docker build -t diegorubin/rabbitmq-to-kafka-base .

clean-docker:
	docker images diegorubin/rabbitmq-to-kafka-base -q | xargs -r docker rmi -f
clean: clean-docker

