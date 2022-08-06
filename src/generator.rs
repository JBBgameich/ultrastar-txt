use crate::structs::*;
use thiserror::Error;

/// Result produced by the generator
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur while generating
#[derive(Error, Debug)]
pub enum Error {
    /// the path encoding is invalid
    #[error("invalid path encoding on tag: {tag:?}")]
    InvalidPathEncoding {
        /// tag on which the error occured
        tag: &'static str
    }
}

/// Converts a Song back to the Ultrastar Song format and returns it as a String
///
/// # Arguments
/// * header - the Header struct of the song
/// * lines - a vector of the songs lines
///
pub fn generate_song_txt(header: &Header, lines: &[Line]) -> Result<String> {
    // generate header
    let mp3_str = header.audio_path.clone();
    /*let mp3_str = match Some(header.audio_path) {
        Some(x) => x,
        None => Err(Error::InvalidPathEncoding("MP3")),
    }; */
    let mut song_txt_str = format!(
        "#TITLE:{}\n#ARTIST:{}\n#MP3:{}\n#BPM:{}\n",
        header.title, header.artist, mp3_str, header.bpm
    );
    if let Some(gap) = header.gap {
        song_txt_str.push_str(&format!("#GAP:{}\n", gap));
    }
    if let Some(cover_path) = header.cover_path.clone() {
        song_txt_str.push_str(&format!("#COVER:{}\n", cover_path));
    }
    if let Some(background_path) = header.background_path.clone() {
        song_txt_str.push_str(&format!("#BACKGROUND:{}\n", background_path));
    }
    if let Some(video_path) = header.video_path.clone() {
        song_txt_str.push_str(&format!("#VIDEO:{}\n", video_path));
    }
    if let Some(videogap) = header.video_gap {
        song_txt_str.push_str(&format!("#VIDEOGAP:{}\n", videogap));
    }
    if let Some(genre) = header.genre.clone() {
        song_txt_str.push_str(&format!("#GENRE:{}\n", genre));
    }
    if let Some(edition) = header.edition.clone() {
        song_txt_str.push_str(&format!("#EDITION:{}\n", edition));
    }
    if let Some(language) = header.language.clone() {
        song_txt_str.push_str(&format!("#LANGUAGE:{}\n", language));
    }
    if let Some(year) = header.year {
        song_txt_str.push_str(&format!("#YEAR:{}\n", year));
    }
    if let Some(relative) = header.relative {
        if relative {
            song_txt_str.push_str("#RELATIVE:YES\n");
        } else {
            song_txt_str.push_str("#RELATIVE:NO\n");
        }
    }
    if let Some(unknown) = header.unknown.clone() {
        for (key, value) in unknown.iter() {
            song_txt_str.push_str(&format!("#{}:{}\n", key, value));
        }
    }

    // generate lines
    for line in lines.iter() {
        if line.start != 0 {
            if line.rel.is_some() {
                song_txt_str.push_str(format!("- {} {}\n", line.start, line.rel.unwrap()).as_ref());
            } else {
                song_txt_str.push_str(format!("- {}\n", line.start).as_ref());
            }
        }
        for note in line.notes.iter() {
            match *note {
                Note::Regular {
                    start,
                    duration,
                    pitch,
                    ref text,
                } => song_txt_str
                    .push_str(format!(": {} {} {} {}\n", start, duration, pitch, text).as_ref()),
                Note::Golden {
                    start,
                    duration,
                    pitch,
                    ref text,
                } => song_txt_str
                    .push_str(format!("* {} {} {} {}\n", start, duration, pitch, text).as_ref()),
                Note::Freestyle {
                    start,
                    duration,
                    pitch,
                    ref text,
                } => song_txt_str
                    .push_str(format!("F {} {} {} {}\n", start, duration, pitch, text).as_ref()),
                Note::PlayerChange { player } => {
                    song_txt_str.push_str(format!("P{}\n", player).as_ref())
                }
            };
        }
    }
    song_txt_str.push_str("E");
    Ok(song_txt_str)
}
