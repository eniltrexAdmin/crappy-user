SHELL=/bin/bash
PROJECT_DIRECTORY := $(shell pwd)

local-install:
	cd cicd/build/; docker-compose -f docker-compose-local.yml up -d;
	export DATABASE_URL="postgres://postgres:postgres@localhost:5432/crappy-user"; sqlx migrate --source src/infrastructure/persistence/postgres/migrations run;

local-start:
	cd cicd/build/; docker-compose -f docker-compose-local.yml up -d;

local-run:
	set -a; source .env.local; set +a; cargo run;

local-functional-test:
	set -a; source .env.local; set +a; cargo test;
