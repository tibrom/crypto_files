use std::fmt::format;

use crate::domain::models::{Command, CommandError, Credentials};
use crate::domain::services::{
    BaseActions, ConsoleError, CryptoService, FileService, TerminalService,
};

pub struct ReadUseCase<F, C, T> {
    is_first_chunk: bool,
    is_encrypted: bool,
    file_service: F,
    crypto_service: C,
    terminal: T,
}

impl<F, C, T> ReadUseCase<F, C, T>
where
    F: FileService,
    C: CryptoService,
    T: TerminalService,
{
    pub fn new(file_service: F, crypto_service: C, terminal: T) -> Self {
        Self {
            is_first_chunk: true,
            is_encrypted: false,
            file_service,
            crypto_service,
            terminal,
        }
    }
}

impl<F, C, T> BaseActions for ReadUseCase<F, C, T>
where
    F: FileService,
    C: CryptoService,
    T: TerminalService,
{
    type Error = CommandError<F::Error, C::Error>;
    fn execute(&mut self) -> Result<(), Self::Error> {
        self.file_service
            .init_original()
            .map_err(CommandError::FileServiceError)?;
        loop {
            let chunk = self.file_service.read_chunk_original();
            if chunk.is_empty() {
                break;
            }

            if self.is_first_chunk {
                self.is_first_chunk = false;
                self.is_encrypted = self
                    .crypto_service
                    .is_encrypt(&chunk)
                    .map_err(CommandError::CryptoService)?;
            }

            let clear_chunk = if self.is_encrypted {
                self.crypto_service
                    .decrypt(chunk)
                    .map_err(CommandError::CryptoService)
                    .map_err(|e| match self.file_service.revert() {
                        Ok(_) => e,
                        Err(err) => CommandError::FileServiceError(err),
                    })?
            } else {
                chunk
            };
            self.terminal.print_chunk(clear_chunk);
        }
        Ok(())
    }
}
