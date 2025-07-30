all: up

up:
	docker-compose up --build

down:
	docker-compose down

restart:
	docker-compose down && docker-compose up --build

local:
	docker run --name pg \
	-e POSTGRES_USER=postgres \
	-e POSTGRES_PASSWORD=password \
	-e POSTGRES_DB=mydb \
	-e LANG=en_US.UTF-8 \
	-e LC_ALL=en_US.UTF-8 \
	-e LC_MESSAGES=en_US.UTF-8 \
	-p 5432:5432 \
	-d postgres:17
	cargo run

.PHONY: all up down restart