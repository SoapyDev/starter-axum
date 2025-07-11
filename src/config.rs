#[cfg(feature = "ssr")]
pub mod ssr {
    use secrecy::SecretString;
    use serde::Deserialize;
    use serde_aux::field_attributes::deserialize_number_from_string;

    #[derive(Deserialize, Clone, Debug)]
    pub struct Settings {
        pub application: ApplicationSettings,
        pub web_uri: String,
        pub jwt: JwtSettings,
        pub tracing_level: String,
    }

    #[derive(Deserialize, Clone, Debug)]
    pub struct ApplicationSettings {
        #[serde(deserialize_with = "deserialize_number_from_string")]
        pub port: u16,
        pub host: String,
        pub hmac_secret: SecretString,
    }

    #[derive(Deserialize, Clone, Debug)]
    pub struct JwtSettings {
        pub secret: SecretString,
        #[serde(deserialize_with = "deserialize_number_from_string")]
        pub max_age: i64,
    }

    pub fn get_configuration() -> Result<Settings, config::ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        let configuration_directory = base_path.join("./configuration");

        // Default to local
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT.");

        let environment_filename = format!("{}.yaml", environment.as_str());

        let settings = config::Config::builder()
            .add_source(config::File::from(
                configuration_directory.join("base.yaml"),
            ))
            .add_source(config::File::from(
                configuration_directory.join(environment_filename),
            ))
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;
        settings.try_deserialize::<Settings>()
    }

    pub enum Environment {
        Local,
        Production,
    }

    impl Environment {
        pub fn as_str(&self) -> &'static str {
            match self {
                Environment::Local => "local",
                Environment::Production => "production",
            }
        }
    }

    impl TryFrom<String> for Environment {
        type Error = String;

        fn try_from(s: String) -> Result<Self, Self::Error> {
            match s.to_lowercase().as_str() {
                "local" => Ok(Self::Local),
                "production" => Ok(Self::Production),
                other => Err(format!(
                    "{} is not supported. Use either 'local' or 'production'.",
                    other
                )),
            }
        }
    }
}
