use std::fs::File;
use std::path::PathBuf;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};

pub struct SoundFacade {
  stream: OutputStream,
  stream_handle: OutputStreamHandle,
}

impl SoundFacade {
  pub fn new() -> Self {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();

    SoundFacade {
      stream,
      stream_handle,
    }
  }

  pub fn play_sound(&self, file_path: PathBuf) {
    let source = Decoder::new(File::open(file_path).unwrap()).unwrap();
    let _ = self.stream_handle.play_raw(source.convert_samples());
  }
}
