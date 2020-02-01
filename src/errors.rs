use std::path::{Path, PathBuf};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        /// Io Error
        Io(err: std::io::Error) {
            from()
            cause(err)
            display("I/O Error: {}", err)
        }
        /// File IO Error
        File(filename: PathBuf, err: std::io::Error) {
            context(path: &'a Path, err: std::io::Error)
                -> (path.to_path_buf(), err)
        }
        /// Int Parse Error
        ParseInt(err: std::num::ParseIntError) {
            from()
            cause(err)
            display("Parse int error: {}", err)
        }
        /// note mark range invalid
        NoteRange(desc: String) {
            display("Note range format error: {}", desc)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
