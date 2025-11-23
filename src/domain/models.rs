use super::services::ConsoleError;
pub enum CommandError<F, C> {
    FileServiceError(F),
    CryptoService(C),
}

impl<F, C> ConsoleError for CommandError<F, C>
where
    F: ConsoleError,
    C: ConsoleError,
{
    fn consol_log(&self) -> String {
        match self {
            CommandError::CryptoService(c) => c.consol_log(),
            CommandError::FileServiceError(c) => c.consol_log(),
        }
    }
}

/// Команды, которые получает программа
pub enum Command {
    Prepare(String),
    Read(String),
    Decrypt(String),
}

/// Настройки системы
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Credentials {
    #[serde(deserialize_with = "hex_vec::deserialize")]
    pub key: Vec<u8>,
    pub chunk_size: usize,
}

mod hex_vec {
    use serde::Deserialize;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        hex::decode(&s).map_err(serde::de::Error::custom)
    }
}
