pub mod controllers;

use crate::configuration::Settings;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::{postgres, ConnectOptions, Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use tracing::log;
use tracing_actix_web::TracingLogger;

pub struct CrappyUserApp {
    settings: Settings,
}

impl CrappyUserApp {
    pub fn new(configuration: Settings) -> Self {
        Self {
            settings: configuration,
        }
    }

    fn pg_connect_options(&self) -> postgres::PgConnectOptions {
        let ssl_mode = match self.settings.database.require_ssl {
            true => postgres::PgSslMode::Require,
            false => postgres::PgSslMode::Prefer,
        };

        let log_level: log::LevelFilter =
            match self.settings.database.log_level.to_lowercase().as_str() {
                "error" => log::LevelFilter::Error,
                "debug" => log::LevelFilter::Debug,
                "info" => log::LevelFilter::Info,
                "off" => log::LevelFilter::Off,
                "trace" => log::LevelFilter::Trace,
                "warn" => log::LevelFilter::Warn,
                _ => log::LevelFilter::Off,
            };

        let mut options = postgres::PgConnectOptions::new()
            .host(&self.settings.database.host)
            .username(&self.settings.database.username)
            .password(&self.settings.database.password.expose_secret())
            .port(self.settings.database.port)
            .ssl_mode(ssl_mode);
        options.log_statements(log_level);
        options
    }

    fn pg_connect_options_with_db(&self) -> postgres::PgConnectOptions {
        self.pg_connect_options()
            .database(&self.settings.database.database_name)
    }

    pub async fn create_database_if_not_exists(&self) {
        let mut connection = PgConnection::connect_with(&self.pg_connect_options())
            .await
            .expect("Failed to connect to Postgres");
        connection
            .execute(
                format!(
                    r#"CREATE DATABASE "{}";"#,
                    self.settings.database.database_name
                )
                .as_str(),
            )
            .await
            .expect("Failed to create database");
    }

    pub async fn remove_database(&self) {
        let mut connection = PgConnection::connect_with(&self.pg_connect_options())
            .await
            .expect("Failed to connect to Postgres");
        connection
            .execute(
                format!(
                    r#"DROP DATABASE "{}";"#,
                    self.settings.database.database_name
                )
                .as_str(),
            )
            .await
            .expect("Failed to drop database");
    }

    pub async fn migrate_database(&self) {
        let connection_pool = PgPool::connect_with(self.pg_connect_options_with_db())
            .await
            .expect("Failed to connect to Postgres");
        sqlx::migrate!("./src/infrastructure/persistence/postgres/migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database");
    }

    pub fn get_connection_pool(&self) -> PgPool {
        postgres::PgPoolOptions::new()
            .connect_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(self.pg_connect_options_with_db())
    }

    pub async fn build_actix_server(&self) -> Result<ApplicationReady, std::io::Error> {
        let connection_pool = PgPoolOptions::new()
            .connect_timeout(std::time::Duration::from_secs(2))
            .connect_with(self.pg_connect_options_with_db())
            .await
            .expect("Failed to connect to Postgres.");

        let db_pool = web::Data::new(connection_pool);

        let address = format!(
            "{}:{}",
            self.settings.application.host, self.settings.application.port
        );

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        //use move keyword
        // the main think I dislike defining all the routes in here.
        let server = HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .route("/health_check", web::get().to(controllers::health_check))
                // .route("/register", web::post().to(controllers::post_match_request))
                // .route("/authenticate", web::get().to(controllers::get_match_requests))
                .app_data(db_pool.clone())
        })
        .listen(listener)?
        .run();

        Ok(ApplicationReady { server, port })
    }
}

pub struct ApplicationReady {
    server: Server,
    port: u16,
}
impl ApplicationReady {
    pub async fn run_actix_server_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn stop_server(self) {}
}
