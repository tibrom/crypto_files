use crate::domain::models::Credentials;
use crate::domain::services::ConsoleError;
use config::{Config, ConfigError};

const MIN_KEY_LEN: usize = 32;

pub enum CredentialsLoaderError {
    LoadingError(ConfigError),
    DeserializeError(ConfigError),
    InvalidKeyLength(usize),
}

impl ConsoleError for CredentialsLoaderError {
    fn consol_log(&self) -> String {
        match self {
            CredentialsLoaderError::LoadingError(e) => format!(
                "Ошибка загрузки конфигурации {e}, убедитесь что файл credentials.toml, находится радом с программой и в нем существуют поля key и chunk_size",
            ),
            CredentialsLoaderError::DeserializeError(e) => {
                format!("Ошибка десериализации файла credentials.toml: {e}")
            }
            CredentialsLoaderError::InvalidKeyLength(v) => {
                format!("Недостаточная длина ключа KEY. Нужно 32 байта, имеется {v}.")
            }
        }
    }
}

pub struct CredentialsLoader;

impl CredentialsLoader {
    pub fn try_load() -> Result<Credentials, CredentialsLoaderError> {
        let config = Config::builder()
            .add_source(config::File::with_name("credentials"))
            .add_source(config::Environment::with_prefix("CRYPTO"))
            .build()
            .map_err(CredentialsLoaderError::LoadingError)?;

        let creds = config
            .try_deserialize::<Credentials>()
            .map_err(CredentialsLoaderError::DeserializeError)?;

        let creds_key_len = creds.key.len();

        if creds_key_len < MIN_KEY_LEN {
            return Err(CredentialsLoaderError::InvalidKeyLength(creds_key_len));
        }

        Ok(creds)
    }
}
