use std::path::PathBuf;

use crate::domain::models::{Command, CommandError, Credentials};
use crate::domain::services::BaseActions;

use crate::application::decrypt_use_case::DecryptUseCase;
use crate::application::prepare_use_case::PrepareUseCase;
use crate::application::read_use_case::ReadUseCase;
use crate::infrastructure::crypto_service::{AesCtrCryptoService, CryptoError};
use crate::infrastructure::file_service::{FsError, LocalFileService};
use crate::infrastructure::terminal_service::Terminal;

pub struct CommandExecutor {
    config: Credentials,
}

impl CommandExecutor {
    pub fn new(config: Credentials) -> Self {
        Self { config }
    }

    fn file_service(&self, path: String) -> LocalFileService {
        let path_buf = PathBuf::from(path);
        LocalFileService::new(self.config.chunk_size, path_buf)
    }

    fn crypto_service(&self) -> AesCtrCryptoService {
        AesCtrCryptoService::from(self.config.clone())
    }

    pub fn run_command(&self, command: Command) -> Result<(), CommandError<FsError, CryptoError>> {
        match command {
            Command::Prepare(path) => {
                PrepareUseCase::new(self.file_service(path), self.crypto_service(), Terminal)
                    .execute()
            }
            Command::Decrypt(path) => {
                DecryptUseCase::new(self.file_service(path), self.crypto_service(), Terminal)
                    .execute()
            }
            Command::Read(path) => {
                ReadUseCase::new(self.file_service(path), self.crypto_service(), Terminal).execute()
            }
        }
    }
}
