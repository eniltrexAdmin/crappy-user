SHELL=/bin/bash
PROJECT_DIRECTORY := $(shell pwd)

local-install:
	echo "Starting POSTGRESQL container and executing migrations against it. Command idempotent, but really, execute it once.";
	cd cicd/build/; docker-compose -f docker-compose-local.yml up --renew-anon-volumes --remove-orphans -d; sleep 5;
	export DATABASE_URL="postgres://postgres:postgres@localhost:5432/crappy-user"; sqlx migrate --source src/infrastructure/persistence/postgres/migrations run;

local-start:
	cd cicd/build/; docker-compose -f docker-compose-local.yml up -d;

local-stop:
	cd cicd/build/; docker-compose -f docker-compose-local.yml stop;

local-run:
	set -a; source .env.local; set +a; cargo run;

local-functional-test:
	set -a; source .env.local; set +a; cargo test;
