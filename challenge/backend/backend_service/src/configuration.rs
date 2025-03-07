use config::Environment;
#[derive(serde::Deserialize)]
pub struct Settings {
    pub port: u16,
    pub host: String,
    pub log_level: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration rader
    let mut settings = config::Config::default();

    // Add configuration values from a file named `configuration`.
    // It will look for any top-level file with an extension
    // that `config` knows how to parse: yaml, json, etc.
    settings.merge(config::File::with_name("configuration"))?;

    // Add in settings from the environment (with a prefix of APP)
    // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
    settings.merge(Environment::with_prefix("app"))?;

    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_into()
}
