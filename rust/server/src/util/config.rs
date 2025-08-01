use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
	pub(crate) server_config: ServerConfig,
	pub(crate) postgresql_config: Option<PostgreSQLConfig>,
}

#[derive(Deserialize)]
pub(crate) struct ServerConfig {
	pub(crate) host: Option<String>,  // Optional in TOML, can be overridden by env
	pub(crate) port: Option<u16>,     // Optional in TOML, can be overridden by env
}

impl ServerConfig {
	pub(crate) fn get_host(&self) -> String {
		std::env::var("VSS_SERVER_HOST")
			.ok()
			.or_else(|| self.host.clone())
			.expect("Server host must be provided in config or env var VSS_SERVER_HOST must be set.")
	}

	pub(crate) fn get_port(&self) -> u16 {
		std::env::var("VSS_SERVER_PORT")
			.ok()
			.and_then(|p| p.parse().ok())
			.or(self.port)
			.expect("Server port must be provided in config or env var VSS_SERVER_PORT must be set.")
	}
}

#[derive(Deserialize)]
pub(crate) struct PostgreSQLConfig {
	pub(crate) username: Option<String>, // Optional in TOML, can be overridden by env
	pub(crate) password: Option<String>, // Optional in TOML, can be overridden by env
	pub(crate) host: Option<String>,     // Optional in TOML, can be overridden by env
	pub(crate) port: Option<u16>,        // Optional in TOML, can be overridden by env
	pub(crate) database: Option<String>,  // Optional in TOML, can be overridden by env
}

impl PostgreSQLConfig {
	pub(crate) fn to_connection_string(&self) -> String {
		let username_env = std::env::var("VSS_POSTGRESQL_USERNAME");
		let username = username_env.as_ref()
			.ok()
			.or_else(|| self.username.as_ref())
			.expect("PostgreSQL database username must be provided in config or env var VSS_POSTGRESQL_USERNAME must be set.");
		let password_env = std::env::var("VSS_POSTGRESQL_PASSWORD");
		let password = password_env.as_ref()
			.ok()
			.or_else(|| self.password.as_ref())
			.expect("PostgreSQL database password must be provided in config or env var VSS_POSTGRESQL_PASSWORD must be set.");
		let host = std::env::var("VSS_POSTGRESQL_HOST")
			.ok()
			.or_else(|| self.host.clone())
			.expect("PostgreSQL database host must be provided in config or env var VSS_POSTGRESQL_HOST must be set.");
		let port = std::env::var("VSS_POSTGRESQL_PORT")
			.ok()
			.and_then(|p| p.parse().ok())
			.or(self.port)
			.expect("PostgreSQL database port must be provided in config or env var VSS_POSTGRESQL_PORT must be set.");
		let database = std::env::var("VSS_POSTGRESQL_DATABASE")
			.ok()
			.or_else(|| self.database.clone())
			.expect("PostgreSQL database name must be provided in config or env var VSS_POSTGRESQL_DATABASE must be set.");

		format!(
			"postgresql://{}:{}@{}:{}/{}",
			username, password, host, port, database
		)
	}
}

pub(crate) fn load_config(config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
	let config_str = std::fs::read_to_string(config_path)?;
	let config: Config = toml::from_str(&config_str)?;
	Ok(config)
}
