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
        /// HTTP reqwest Error
        Reqwest(err: reqwest::Error) {
            from()
            cause(err)
            display("Reqwest Error: {}", err)
        }
        /// serde error
        SerdeJSON(err: serde_json::Error) {
            from()
            cause(err)
            display("srde_json error: {}", err)
        }
        /// HTTP reqwest Invalid Header Value.
        InvalidHeaderValue(err: reqwest::header::InvalidHeaderValue) {
            from()
            cause(err)
            display("Reqwest Invalid Header: {}", err)
        }
        /// note mark range invalid
        NoteRange(desc: String) {
            display("Note range format error: {}", desc)
        }
        /// Invalid Response.
        InvalidResponse(url: String, resp: reqwest::blocking::Response) {
            display("{} invalid response: {:?}", url, resp)
        }
        /// Invalid Response Metadata Json.
        InvalidMetaJson(value: String) {
            display("invalid meta data json: {}", value)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
