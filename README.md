# RUST BASIC MICRO

Base micro to start working;

* Rust
* Actix
* Telemetry from zero 2 production packages
* UUID even though its not used.
* PGSQL with pg package
* Makefile and docker compose with postgre file
* configuration via env variables (.env.local example present)

## Usage

Start the micro via Makefile:
`make local-start`

You can access the health_check endpoint: 127.0.0.1:8001/health_check

Change the .env.local variables as wanted, if you change the prefix
make sure you also change it in the configuration.rs file.

## Extend

missing the cicd bits

