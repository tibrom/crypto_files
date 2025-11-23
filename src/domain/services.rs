// Отвечает за чтение файла по частям и удаление его
pub trait FileService {
    type Error: ConsoleError;
    /// Создает временный файл с исходными данными
    fn init_original(&mut self) -> Result<&mut Self, Self::Error>;
    fn make_temp(&mut self) -> Result<&mut Self, Self::Error>;
    fn rename_temp_as_original(&mut self) -> Result<&mut Self, Self::Error>;
    fn delete_original(&mut self) -> Result<&mut Self, Self::Error>;
    fn revert(&mut self) -> Result<&mut Self, Self::Error>;

    fn read_chunk_original(&mut self) -> Vec<u8>;
    fn write_chunk(&mut self, chunk: Vec<u8>) -> Result<(), Self::Error>;
}

/// Отвечает за шифрование и дешифрование данных
pub trait CryptoService {
    type Error: ConsoleError;
    fn is_encrypt(&mut self, chunk: &Vec<u8>) -> Result<bool, Self::Error>;
    fn encrypt(&mut self, chunk: Vec<u8>) -> Result<Vec<u8>, Self::Error>;
    fn decrypt(&mut self, chunk: Vec<u8>) -> Result<Vec<u8>, Self::Error>;
}

/// Отвечает за строковое представление ошибки
pub trait ConsoleError {
    fn consol_log(&self) -> String;
}

pub trait BaseActions {
    type Error;
    fn execute(&mut self) -> Result<(), Self::Error>;
}

/// Отвечает за отображение сообщений в терминале
pub trait TerminalService {
    fn print_msg(&self, msg: String);
    fn print_error_msg(&self, msg: String);
    fn print_chunk(&self, value: Vec<u8>);
    fn print_message_in_line(&self, msg: String);
}
