use crate::domain::services::{ConsoleError, CryptoService, FileService};

pub struct MockError(String);

impl ConsoleError for MockError {
    fn consol_log(&self) -> String {
        self.0.clone()
    }
}

impl From<&str> for MockError {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

pub struct MockedFileService<'a> {
    pub called_method: Vec<&'a str>,
    pub ok_init_original: bool,
    pub ok_make_temp: bool,
    pub ok_rename_temp_as_original: bool,
    pub ok_delete_original: bool,
    pub ok_revert: bool,
    pub ok_write_chunk: bool,
    pub read_chunks: Vec<Vec<u8>>,
    pub write_chunk: Vec<Vec<u8>>,
}

impl<'a> MockedFileService<'a> {
    pub fn new() -> Self {
        Self {
            called_method: Vec::new(),

            ok_init_original: true,
            ok_make_temp: true,
            ok_rename_temp_as_original: true,
            ok_delete_original: true,
            ok_revert: true,
            ok_write_chunk: true,

            read_chunks: vec![],
            write_chunk: vec![],
        }
    }
}

impl<'a> FileService for MockedFileService<'a> {
    type Error = MockError;

    fn init_original(&mut self) -> Result<&mut Self, Self::Error> {
        self.called_method.push("init_original");
        if self.ok_init_original {
            Ok(self)
        } else {
            Err("MockedFileService.init_original".into())
        }
    }

    fn make_temp(&mut self) -> Result<&mut Self, Self::Error> {
        self.called_method.push("make_temp");
        if self.ok_make_temp {
            Ok(self)
        } else {
            Err("MockedFileService.make_temp".into())
        }
    }

    fn rename_temp_as_original(&mut self) -> Result<&mut Self, Self::Error> {
        self.called_method.push("rename_temp_as_original");
        if self.ok_rename_temp_as_original {
            Ok(self)
        } else {
            Err("MockedFileService.rename_temp_as_original".into())
        }
    }

    fn delete_original(&mut self) -> Result<&mut Self, Self::Error> {
        self.called_method.push("delete_original");
        if self.ok_delete_original {
            Ok(self)
        } else {
            Err("MockedFileService.delete_original".into())
        }
    }

    fn revert(&mut self) -> Result<&mut Self, Self::Error> {
        self.called_method.push("revert");
        if self.ok_revert {
            Ok(self)
        } else {
            Err("MockedFileService.revert".into())
        }
    }

    fn read_chunk_original(&mut self) -> Vec<u8> {
        self.called_method.push("read_chunk_original");

        if self.read_chunks.is_empty() {
            return vec![];
        }

        self.read_chunks.remove(0)
    }

    fn write_chunk(&mut self, chunk: Vec<u8>) -> Result<(), Self::Error> {
        self.called_method.push("write_chunk");

        if self.ok_write_chunk {
            self.write_chunk.push(chunk);
            Ok(())
        } else {
            Err("MockedFileService.write_chunk".into())
        }
    }
}

pub struct MockCryptoService {
    pub ok_decrypt: bool,
    pub ok_encrypt: bool,
    pub ok_is_encrypt: bool,
    pub is_encrypt: bool,
    pub encrypt_chunks: Vec<Vec<u8>>,
    pub decrypt_chunk: Vec<Vec<u8>>,
}

impl MockCryptoService {
    pub fn new() -> Self {
        Self {
            ok_decrypt: true,
            ok_encrypt: true,
            ok_is_encrypt: true,
            is_encrypt: true,
            encrypt_chunks: vec![],
            decrypt_chunk: vec![],
        }
    }
}

impl CryptoService for MockCryptoService {
    type Error = MockError;
    fn decrypt(&mut self, _chunk: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
        let result = self.decrypt_chunk[0].clone();
        if self.ok_decrypt {
            return Ok(result);
        }
        Err(MockError::from("MockCryptoService.decrypt"))
    }
    fn encrypt(&mut self, _chunk: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
        let result = self.encrypt_chunks[0].clone();
        if self.ok_encrypt {
            return Ok(result);
        }
        Err(MockError::from("MockCryptoService.encrypt"))
    }
    fn is_encrypt(&mut self, _chunk: &Vec<u8>) -> Result<bool, Self::Error> {
        if self.ok_is_encrypt {
            return Ok(self.is_encrypt);
        }
        Err(MockError::from("MockCryptoService.is_encrypt"))
    }
}
