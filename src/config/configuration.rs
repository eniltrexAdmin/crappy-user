use secrecy::Secret;

const ENVIRONMENT_PREFIX: &str = "CRAPPYUSER";
const ENVIRONMENT_SEPARATOR: &str = "__";

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Debug)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
    pub log_level: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let _environment: Environment = std::env::var(format!(
        "{}{}{}",
        &ENVIRONMENT_PREFIX, &ENVIRONMENT_SEPARATOR, "ENVIRONMENT"
    ))
    .unwrap_or_else(|_| "local".into())
    .try_into()
    .expect("Failed to parse CRAPPY-USER_ENVIRONMENT.");

    let log_level = std::env::var(format!(
        "{}{}{}",
        &ENVIRONMENT_PREFIX, &ENVIRONMENT_SEPARATOR, "CRAPPY__DATABASE__LOG_LEVEL"
    ))
    // .unwrap_or_else(|_| String::from("trace"));
    .unwrap_or_default();

    config::Config::builder()
        .add_source(config::Environment::with_prefix(&ENVIRONMENT_PREFIX).separator("__"))
        .set_override("database.log_level", log_level)?
        .build()?
        .try_deserialize()
}

//I leave it here since this "try_into()" when this implements try_from is very interesting!
pub enum Environment {
    Local,
    Production,
    DockerCompose,
}
impl Environment {
    pub fn file_config(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::DockerCompose => "docker-compose-config",
            Environment::Production => "production",
        }
    }
}
impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "docker-compose" => Ok(Self::DockerCompose),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}
