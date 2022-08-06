extern crate chardet;
extern crate encoding;

use crate::parser::{parse_txt_header_str, parse_txt_lines_str};
use crate::structs::TXTSong;
use std::fs::File;
use std::io::{Read, self};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Result produced by the loader
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur while loading a file
#[derive(Error, Debug)]
pub enum Error {
    /// input output error while handling the file
    #[error("io error")]
    IOError(io::Error),

    /// error in encoding detection
    #[error("encoding detection error")]
    EncodingDetectionError,

    /// error while decoding
    #[error("decoding error: {msg:?}")]
    DecodingError {
        /// message describing the decoding error
        msg: String
    },

    /// error in path canonicalization
    #[error("canonicalization error")]
    CanonicalizationError(io::Error),

    /// error in parsing the song header
    #[error("header parsing error")]
    HeaderParsingError(crate::parser::Error),

    /// error in parsing the songs lines
    #[error("lines parsing error")]
    LinesParsingError(crate::parser::Error)
}


fn read_file_to_string<P: AsRef<Path>>(p: P) -> Result<String> {
    let p = p.as_ref();
    let mut f = File::open(p).map_err(Error::IOError)?;
    let mut reader: Vec<u8> = Vec::new();
    f.read_to_end(&mut reader)
        .map_err(Error::IOError)?;

    // detect encoding and decode to String
    let chardet_result = chardet::detect(&reader);
    let whtwg_label = chardet::charset2encoding(&chardet_result.0);
    let coder = encoding::label::encoding_from_whatwg_label(whtwg_label);
    let file_content = match coder {
        Some(c) => match c.decode(&reader, encoding::DecoderTrap::Ignore) {
            Ok(x) => x,
            Err(e) => {
                return Err(Error::DecodingError { msg: e.into_owned() })
            },
        },
        None => return Err(Error::EncodingDetectionError),
    };

    Ok(file_content)
}

fn canonicalize_path(path: String, base_path: impl AsRef<Path>) -> Result<String> {
    fn perform_canonicalization<P: AsRef<Path>, B: AsRef<Path>>(
        path: &Option<P>,
        base_path: B,
    ) -> Result<Option<PathBuf>> {

        Ok(if let Some(ref path) = path {
            let mut tmp_path = PathBuf::from(base_path.as_ref());
            tmp_path.push(path);
            let result = tmp_path
                .canonicalize()
                .map_err(Error::CanonicalizationError)?;
            Some(result)
        } else {
            None
        })
    }

    if path_is_local(&path) && !path.starts_with("file:///") {
        let path = if path.starts_with("file://") {
            path.chars().skip("file://".len()).collect()
        } else {
            path.clone()
        };
        let path = PathBuf::from(path);
        Ok(perform_canonicalization(&Some(path), base_path)?.unwrap().display().to_string())
    } else {
        Ok(path)
    }
}

/// Takes path to a song file and returns TXTSong struct with canonicalized paths
///
/// # Arguments
/// * path - the path to the song file to parse
///
pub fn parse_txt_song<P: AsRef<Path>>(path: P) -> Result<TXTSong> {
    let path = path.as_ref();
    let txt = read_file_to_string(path)?;

    let mut txt_song = TXTSong {
        header: parse_txt_header_str(txt.as_ref()).map_err(Error::HeaderParsingError)?,
        lines: parse_txt_lines_str(txt.as_ref()).map_err(Error::LinesParsingError)?,
    };

    // canonicalize paths
    if let Some(base_path) = path.parent() {
        txt_song.header.audio_path = canonicalize_path(txt_song.header.audio_path, base_path)?;

        if let Some(video_path) = txt_song.header.video_path {
            txt_song.header.video_path = Some(canonicalize_path(video_path, base_path)?);
        }
        if let Some(cover_path) = txt_song.header.cover_path {
            txt_song.header.cover_path = Some(canonicalize_path(cover_path, base_path)?);
        }
        if let Some(background_path) = txt_song.header.background_path {
            txt_song.header.background_path = Some(canonicalize_path(background_path, base_path)?);
        }
    }

    Ok(txt_song)
}

/// Returns whether the path references a local file.
pub fn path_is_local(path: &str) -> bool {
    // guess based on the occurence of a ://, but not a file://
    if path.contains("://") && !path.starts_with("file://") {
        false
    } else {
        true
    }
}
