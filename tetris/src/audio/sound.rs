use std::fmt;
use std::{fs::File, io};
use std::path::PathBuf;
use rodio::decoder::DecoderError;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source, StreamError, PlayError};

pub struct SoundFacade {
  _stream: OutputStream,
  stream_handle: OutputStreamHandle,
}
#[derive(Debug)]
pub enum SoundFacadeError {
  ReadFileError(io::Error),
  DecodeFileError(DecoderError),
  PlayError(PlayError),
  StreamError(StreamError)
}


impl fmt::Display for SoundFacadeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
          SoundFacadeError::ReadFileError(err) => write!(f, "ReadFileError: {}", err),
          SoundFacadeError::DecodeFileError(err) => write!(f, "DecodeFileError: {}", err),
          SoundFacadeError::PlayError(err) => write!(f, "PlayError: {}", err),
          SoundFacadeError::StreamError(err) => write!(f, "StreamError: {}", err),
      }
  }
}


impl SoundFacade {
  pub fn try_default() -> Result<SoundFacade, SoundFacadeError> {
    let (_stream, stream_handle) = OutputStream::try_default().map_err(SoundFacadeError::StreamError)?;

    Ok(SoundFacade {
      _stream,
      stream_handle,
    })
  }

  pub fn play_sound(&self, file_path: PathBuf)  -> Result<(),  SoundFacadeError> {
    let file = File::open(file_path).map_err(SoundFacadeError::ReadFileError)?;
    let source = Decoder::new(file).map_err(SoundFacadeError::DecodeFileError)?;
    self.stream_handle.play_raw(source.convert_samples()).map_err(SoundFacadeError::PlayError)?;
    Ok(())
  }
}
