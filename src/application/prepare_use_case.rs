use crate::domain::models::CommandError;
use crate::domain::services::{BaseActions, CryptoService, FileService, TerminalService};

pub struct PrepareUseCase<F, C, T> {
    counter: usize,
    file_service: F,
    crypto_service: C,
    terminal: T,
}

impl<F, C, T> PrepareUseCase<F, C, T>
where
    F: FileService,
    C: CryptoService,
    T: TerminalService,
{
    pub fn new(file_service: F, crypto_service: C, terminal: T) -> Self {
        Self {
            counter: 0,
            file_service,
            crypto_service,
            terminal,
        }
    }

    fn show_counter(&mut self) {
        self.counter += 1;
        self.terminal
            .print_message_in_line(format!("Шифруется часть {}", self.counter));
    }
}

impl<F, C, T> BaseActions for PrepareUseCase<F, C, T>
where
    F: FileService,
    C: CryptoService,
    T: TerminalService,
{
    type Error = CommandError<F::Error, C::Error>;
    fn execute(&mut self) -> Result<(), Self::Error> {
        self.file_service
            .init_original()
            .and_then(FileService::make_temp)
            .map_err(CommandError::FileServiceError)?;

        loop {
            let chunk = self.file_service.read_chunk_original();
            if chunk.is_empty() {
                break;
            }

            self.show_counter();

            let encrypted = self
                .crypto_service
                .encrypt(chunk)
                .map_err(CommandError::CryptoService)
                .map_err(|e| match self.file_service.revert() {
                    Ok(_) => e,
                    Err(err) => CommandError::FileServiceError(err),
                })?;

            self.file_service
                .write_chunk(encrypted)
                .map_err(CommandError::FileServiceError)
                .map_err(|e| match self.file_service.revert() {
                    Ok(_) => e,
                    Err(err) => CommandError::FileServiceError(err),
                })?;
        }

        self.file_service
            .delete_original()
            .and_then(FileService::rename_temp_as_original)
            .map_err(CommandError::FileServiceError)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::mock_service::{MockCryptoService, MockedFileService};
    use super::*;
    use crate::infrastructure::terminal_service::Terminal;

    // Проверяем что все команды к файловой системе были выполнены в правильной последовательности в
    #[test]
    fn test_normal() {
        let mut file_service = MockedFileService::new();
        file_service.read_chunks = vec![b"abc".to_vec(), b"def".to_vec()];
        let encrypt_chunks = vec![b"encrypt_abc".to_vec(), b"encrypt_def".to_vec()];

        let mut crypto_service = MockCryptoService::new();
        crypto_service.encrypt_chunks = encrypt_chunks.clone();

        let mut use_case = PrepareUseCase::new(file_service, crypto_service, Terminal);
        let result = use_case.execute();

        let command_called = use_case.file_service.called_method;
        let write_chunks = use_case.file_service.write_chunk;

        assert!(result.is_ok());
        assert_eq!(command_called[0], "init_original");
        assert_eq!(command_called[1], "make_temp");
        assert_eq!(command_called[2], "read_chunk_original");
        assert_eq!(command_called[3], "write_chunk");
        assert_eq!(command_called[4], "read_chunk_original");
        assert_eq!(command_called[5], "write_chunk");
        assert_eq!(command_called[6], "read_chunk_original");
        assert_eq!(command_called[7], "delete_original");
        assert_eq!(command_called[8], "rename_temp_as_original");
        assert_eq!(write_chunks[0], encrypt_chunks[0]);
    }

    //Проверяем, что в случае ошибки шифрования вызывается revert
    #[test]
    fn error_encrypt() {
        let mut file_service = MockedFileService::new();
        file_service.read_chunks = vec![b"abc".to_vec(), b"def".to_vec()];
        let encrypt_chunks = vec![b"encrypt_abc".to_vec(), b"encrypt_def".to_vec()];

        let mut crypto_service = MockCryptoService::new();
        crypto_service.encrypt_chunks = encrypt_chunks.clone();
        crypto_service.ok_encrypt = false;

        let use_case = PrepareUseCase::new(file_service, crypto_service, Terminal);

        let command_called = use_case.file_service.called_method;

        assert_eq!(command_called[0], "init_original");
        assert_eq!(command_called[1], "make_temp");
        assert_eq!(command_called[2], "read_chunk_original");
        assert_eq!(command_called[3], "revert");
    }

    #[test]
    fn write_error() {
        let mut file_service = MockedFileService::new();
        file_service.read_chunks = vec![b"abc".to_vec(), b"def".to_vec()];
        file_service.ok_write_chunk = false;

        let encrypt_chunks = vec![b"encrypt_abc".to_vec(), b"encrypt_def".to_vec()];

        let mut crypto_service = MockCryptoService::new();
        crypto_service.encrypt_chunks = encrypt_chunks.clone();

        let use_case = PrepareUseCase::new(file_service, crypto_service, Terminal);

        let command_called = use_case.file_service.called_method;

        assert_eq!(command_called[0], "init_original");
        assert_eq!(command_called[1], "make_temp");
        assert_eq!(command_called[2], "read_chunk_original");
        assert_eq!(command_called[3], "write_chunk");
        assert_eq!(command_called[4], "revert");
    }
}
