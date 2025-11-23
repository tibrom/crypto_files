use std::fs::{File, remove_file, rename};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use uuid::Uuid;

use crate::domain::services::{ConsoleError, FileService};

#[derive(Debug)]
pub enum FsError {
    Io(std::io::Error),
    NoTempFile,
}

impl ConsoleError for FsError {
    fn consol_log(&self) -> String {
        match self {
            FsError::Io(e) => format!("Ошибка файловой системы: {e}"),
            FsError::NoTempFile => "Временный файл отсутствует".to_string(),
        }
    }
}

pub struct LocalFileService {
    original_path: PathBuf,
    temp_path: Option<PathBuf>,
    original_reader: Option<BufReader<File>>,
    tmp_writer: Option<BufWriter<File>>,
    chunk_size: usize,
}

impl LocalFileService {
    pub fn new(chunk_size: usize, path: impl Into<PathBuf>) -> Self {
        Self {
            original_path: path.into(),
            temp_path: None,
            original_reader: None,
            tmp_writer: None,
            chunk_size,
        }
    }
}

impl FileService for LocalFileService {
    type Error = FsError;

    fn init_original(&mut self) -> Result<&mut Self, Self::Error> {
        let original_file = File::open(&self.original_path).map_err(FsError::Io)?;
        self.original_reader = Some(BufReader::new(original_file));
        Ok(self)
    }

    fn make_temp(&mut self) -> Result<&mut Self, Self::Error> {
        let uuid = Uuid::new_v4().to_string();

        let temp_path = self.original_path.with_extension(format!("{uuid}.tmp"));

        //rename(&self.original_path, &temp_path).map_err(FsError::Io)?;

        let new_file = File::create(&temp_path).map_err(FsError::Io)?;

        self.tmp_writer = Some(BufWriter::new(new_file));

        self.temp_path = Some(temp_path);

        Ok(self)
    }

    fn rename_temp_as_original(&mut self) -> Result<&mut Self, Self::Error> {
        let temp_path = self.temp_path.take().ok_or(FsError::NoTempFile)?;
        rename(&temp_path, &self.original_path).map_err(FsError::Io)?;
        Ok(self)
    }

    fn delete_original(&mut self) -> Result<&mut Self, Self::Error> {
        remove_file(&self.original_path).map_err(FsError::Io)?;

        self.tmp_writer = None;

        Ok(self)
    }

    fn read_chunk_original(&mut self) -> Vec<u8> {
        let Some(reader) = self.original_reader.as_mut() else {
            return Vec::new();
        };

        let mut buf = vec![0u8; self.chunk_size];
        match reader.read(&mut buf) {
            Ok(0) | Err(_) => Vec::new(),
            Ok(n) => buf[..n].to_vec(),
        }
    }

    fn write_chunk(&mut self, chunk: Vec<u8>) -> Result<(), Self::Error> {
        let writer = self
            .tmp_writer
            .as_mut()
            .ok_or(FsError::Io(std::io::Error::other("Writer отсутствует")))?;

        writer.write_all(&chunk).map_err(FsError::Io)?;
        writer.flush().map_err(FsError::Io)?;

        Ok(())
    }

    fn revert(&mut self) -> Result<&mut Self, Self::Error> {
        let temp = self.temp_path.take().ok_or(FsError::NoTempFile)?;

        remove_file(&temp).map_err(FsError::Io)?;

        self.tmp_writer = None;

        Ok(self)
    }
}
