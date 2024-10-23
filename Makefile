.PHONY: install build run watch clean db db-stop db-setup db-reset migration migrate migrate-redo migrate-status dev reset setup

DATABASE_URL=postgres://user:password@localhost/feed_a_dev
MIGRATION_DIR=migrations

install:
	cargo install diesel_cli --no-default-features --features postgres
	cargo install cargo-watch
	cargo install --path .

build:
	cargo build

run:
	cargo run

watch:
	cargo watch -x run

clean:
	cargo clean

db:
	docker-compose up -d

db-stop:
	docker-compose down

db-setup:
	diesel setup
	diesel migration run

db-reset:
	diesel database reset

migration:
	@if [ "$(name)" = "" ]; then \
		exit 1; \
	fi
	diesel migration generate $(name)

migrate:
	diesel migration run

migrate-redo:
	diesel migration redo

migrate-status:
	diesel migration list

dev: db db-setup run

reset: clean db-stop db db-reset build run

setup: install db db-setup build run