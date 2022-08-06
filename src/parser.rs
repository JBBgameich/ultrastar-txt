use crate::structs::{Header, Line, Note};
use regex::Regex;
use std::collections::HashMap;
use thiserror::Error;

/// Result produced by the parser
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that occur while parsing
#[derive(Error, Debug)]
pub enum Error {
    /// duplicate header tag was found
    #[error("additional {tag:?} tag found in line: {line:?}")]
    DuplicateHeader{
        /// line on which this error occured
        line: u32,
        /// tag on which this error occured
        tag: &'static str
    },

    /// an essential header is missing
    #[error("essential header is missing")]
    MissingEssential,

    /// value could not be parsed
    #[error("could not parse {line:?} in line: {field:?}")]
    ValueError {
        /// line on which this error occured
        line: u32,
        /// field on which this error occured
        field: &'static str
    },

    /// an unknown note type was found
    #[error("unknown note type in line: {line:?}")]
    UnknownNoteType {
        /// line on which this error occured
        line: u32
    },

    /// could not parse the line at all
    #[error("could not parse line: {line:?}")]
    ParserFailure {
        /// line on which this error occured
        line: u32
    },

    /// song is missing the end terminator
    #[error("missing end indicator")]
    MissingEndIndicator,

    /// song file uses a feature that is not implemented
    #[error("the feature {line:?} in line {feature:?} is not implemented")]
    NotImplemented {
        /// line on which this error occured
        line: u32,
        /// feature that is not implemented
        feature: &'static str
    }
}

/// Parses the Header of a given Ultrastar Song and returns a Header struct
///
/// # Arguments
/// * txt_str  - a &str that contains the song to parse
///
pub fn parse_txt_header_str(txt_str: &str) -> Result<Header> {
    let mut opt_title = None;
    let mut opt_artist = None;
    let mut opt_bpm = None;
    let mut opt_audio_path = None;

    let mut opt_gap = None;
    let mut opt_cover_path = None;
    let mut opt_background_path = None;
    let mut opt_video_path = None;
    let mut opt_video_gap = None;
    let mut opt_genre = None;
    let mut opt_edition = None;
    let mut opt_language = None;
    let mut opt_year = None;
    let mut opt_relative = None;
    let mut opt_unknown: Option<HashMap<String, String>> = None;

    lazy_static! {
        static ref RE: Regex = Regex::new(r"#([A-Z3a-z]*):(.*)").unwrap();
    }

    for (line, line_count) in txt_str.lines().zip(1..) {
        let cap = match RE.captures(line) {
            Some(x) => x,
            None => break,
        };
        let key = cap.get(1).unwrap().as_str();
        let value = cap.get(2).unwrap().as_str();

        if value == "" {
            //TODO: somehow warn about this
            continue;
        }

        match key {
            "TITLE" => {
                if opt_title.is_none() {
                    opt_title = Some(String::from(value));
                } else {
                    return Err(Error::DuplicateHeader { line: line_count, tag: "TITLE" })
                }
            }
            "ARTIST" => {
                if opt_artist.is_none() {
                    opt_artist = Some(String::from(value));
                } else {
                    return Err(Error::DuplicateHeader { line: line_count, tag: "ARTIST"});
                }
            }
            "MP3" => {
                if opt_audio_path.is_none() {
                    opt_audio_path = Some(String::from(value));
                        //Some(PathBuf::from(value));
                } else {
                    return Err(Error::DuplicateHeader { line: line_count, tag: "MP3" });
                }
            }
            "BPM" => {
                if opt_bpm.is_none() {
                    opt_bpm = match value.replace(",", ".").parse() {
                        Ok(x) => Some(x),
                        Err(_) => {
                            return Err(Error::ValueError { line: line_count, field: "BPM" });
                        }
                    };
                } else {
                    return Err(Error::DuplicateHeader { line: line_count, tag: "BPM" });
                }
            }

            // Optional Header fields
            "GAP" => {
                if opt_gap.is_none() {
                    opt_gap = match value.replace(",", ".").parse() {
                        Ok(x) => Some(x),
                        Err(_) => {
                            return Err(Error::ValueError { line: line_count, field: "GAP" });
                        }
                    };
                } else {
                    return Err(Error::DuplicateHeader { line: line_count, tag: "GAP"});
                }
            }
            "COVER" => {
                if opt_cover_path.is_none() {
                    opt_cover_path = Some(String::from(value));
                } else {
                    return Err(Error::DuplicateHeader { line: line_count, tag: "COVER" });
                }
            }
            "BACKGROUND" => {
                if opt_background_path.is_none() {
                    opt_background_path = Some(String::from(value));
                } else {
                    return Err(Error::DuplicateHeader { line: line_count, tag: "BACKGROUND" });
                }
            }
            "VIDEO" => {
                if opt_video_path.is_none() {
                    opt_video_path = Some(String::from(value));
                } else {
                    return Err(Error::DuplicateHeader { line: line_count, tag: "VIDEO" });
                }
            }
            "VIDEOGAP" => {
                if opt_video_gap.is_none() {
                    opt_video_gap = match value.replace(",", ".").parse() {
                        Ok(x) => Some(x),
                        Err(_) => {
                            return Err(Error::ValueError { line: line_count, field: "VIDEOGAP" });
                        }
                    };
                } else {
                    return Err(Error::DuplicateHeader { line: line_count, tag: "VIDEOGAP" });
                }
            }
            "GENRE" => {
                if opt_genre.is_none() {
                    opt_genre = Some(String::from(value));
                } else {
                    return Err(Error::DuplicateHeader { line: line_count, tag: "GENRE" });
                }
            }
            "EDITION" => {
                if opt_edition.is_none() {
                    opt_edition = Some(String::from(value));
                } else {
                    return Err(Error::DuplicateHeader { line: line_count, tag: "EDITION" });
                }
            }
            "LANGUAGE" => {
                if opt_language.is_none() {
                    opt_language = Some(String::from(value));
                } else {
                    return Err(Error::DuplicateHeader { line: line_count, tag: "LANGUAGE" });
                }
            }
            "YEAR" => {
                if opt_year.is_none() {
                    opt_year = match value.parse() {
                        Ok(x) => Some(x),
                        Err(_) => {
                            return Err(Error::ValueError { line: line_count, field: "YEAR" });
                        }
                    };
                } else {
                    return Err(Error::DuplicateHeader { line: line_count, tag: "YEAR" });
                }
            }
            //TODO: check if relative changes line breaks
            "RELATIVE" => {
                if opt_relative.is_none() {
                    opt_relative = match value {
                        "YES" | "yes" => Some(true),
                        "NO" | "no" => Some(false),
                        _ => {
                            return Err(Error::ValueError { line: line_count, field: "RELATIVE"});
                        }
                    }
                } else {
                    return Err(Error::DuplicateHeader { line: line_count, tag: "RELATIVE" });
                }
            }
            // use hashmap to store unknown tags
            k => {
                opt_unknown = match opt_unknown {
                    Some(mut x) => {
                        if !x.contains_key(k) {
                            x.insert(String::from(k), String::from(value));
                            Some(x)
                        } else {
                            return Err(Error::DuplicateHeader { line: line_count, tag: "UNKNOWN" });
                        }
                    }
                    None => {
                        let mut unknown = HashMap::new();
                        unknown.insert(String::from(k), String::from(value));
                        Some(unknown)
                    }
                };
            }
        };
    }

    // build header from Options
    if let (Some(title), Some(artist), Some(bpm), Some(audio_path)) =
        (opt_title, opt_artist, opt_bpm, opt_audio_path)
    {
        let header = Header {
            title,
            artist,
            bpm,
            audio_path,

            gap: opt_gap,
            cover_path: opt_cover_path,
            background_path: opt_background_path,
            video_path: opt_video_path,
            video_gap: opt_video_gap,
            genre: opt_genre,
            edition: opt_edition,
            language: opt_language,
            year: opt_year,
            relative: opt_relative,
            unknown: opt_unknown,
        };
        // header complete
        Ok(header)
    } else {
        // essential field is missing
        Err(Error::MissingEssential)
    }
}

/// Parses the lyric lines of a given Ultarstar song and returns a vector of Line structs
///
/// # Arguments
/// * txt_str  - a &str that contains the song to parse
///
pub fn parse_txt_lines_str(txt_str: &str) -> Result<Vec<Line>> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new("^-\\s?(-?[0-9]+)\\s*$").unwrap();
        static ref LREL_RE: Regex = Regex::new("^-\\s?(-?[0-9]+)\\s+(-?[0-9]+)").unwrap();
        static ref NOTE_RE: Regex =
            Regex::new("^(.)\\s*(-?[0-9]+)\\s+(-?[0-9]+)\\s+(-?[0-9]+)\\s?(.*)").unwrap();
        static ref DUET_RE: Regex = Regex::new("^P\\s?(-?[0-9]+)").unwrap();
    }

    let mut lines_vec = Vec::new();
    let mut current_line = Line {
        start: 0,
        rel: None,
        notes: Vec::new(),
    };

    let mut found_end_indicator = false;
    for (line, line_count) in txt_str.lines().zip(1..) {
        let first_char = match line.chars().nth(0) {
            Some(x) => x,
            None => return Err(Error::ParserFailure { line: line_count }),
        };

        // ignore header
        if first_char == '#' {
            continue;
        }

        // not implemented
        if first_char == 'B' {
            return Err(Error::NotImplemented { line: line_count, feature: "variable bpm" });
        }

        // stop parsing after end symbol
        if first_char == 'E' {
            lines_vec.push(current_line);
            found_end_indicator = true;
            break;
        }

        // current line is a note
        if NOTE_RE.is_match(line) {
            let cap = NOTE_RE.captures(line).unwrap();

            let note_start = match cap.get(2).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(Error::ValueError { line: line_count, field: "note start" });
                }
            };
            let note_duration = match cap.get(3).unwrap().as_str().parse() {
                Ok(x) => {
                    if x >= 0 {
                        x
                    } else {
                        return Err(Error::ValueError { line: line_count, field: "note duration" });
                    }
                }
                Err(_) => {
                    return Err(Error::ValueError { line: line_count, field: "note duration" });
                }
            };
            let note_pitch = match cap.get(4).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(Error::ValueError { line: line_count, field: "note pitch" });
                }
            };
            let note_text = cap.get(5).unwrap().as_str();

            let note = match cap.get(1).unwrap().as_str() {
                ":" => Note::Regular {
                    start: note_start,
                    duration: note_duration,
                    pitch: note_pitch,
                    text: String::from(note_text),
                },
                "*" => Note::Golden {
                    start: note_start,
                    duration: note_duration,
                    pitch: note_pitch,
                    text: String::from(note_text),
                },
                "F" => Note::Freestyle {
                    start: note_start,
                    duration: note_duration,
                    pitch: note_pitch,
                    text: String::from(note_text),
                },
                _ => return Err(Error::UnknownNoteType { line: line_count }),
            };

            current_line.notes.push(note);
            continue;
        }

        // current line is a line break
        if LINE_RE.is_match(line) {
            // push old line to the Line vector and prepare new line
            lines_vec.push(current_line);
            let cap = LINE_RE.captures(line).unwrap();
            let line_start = match cap.get(1).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(Error::ValueError { line: line_count, field: "line start" });
                }
            };
            current_line = Line {
                start: line_start,
                rel: None,
                notes: Vec::new(),
            };
            continue;
        }

        // current line is a relative line break
        if LREL_RE.is_match(line) {
            // push old line to the Line vector and prepare new line
            lines_vec.push(current_line);
            let cap = LREL_RE.captures(line).unwrap();
            let line_start = match cap.get(1).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(Error::ValueError { line: line_count, field: "line start" });
                }
            };
            let line_rel = match cap.get(2).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(Error::ValueError { line: line_count, field: "line rel" });
                }
            };
            current_line = Line {
                start: line_start,
                rel: Some(line_rel),
                notes: Vec::new(),
            };
            continue;
        }

        if DUET_RE.is_match(line) {
            let cap = DUET_RE.captures(line).unwrap();
            let note = match cap.get(1).unwrap().as_str().parse() {
                Ok(x) => {
                    if x >= 1 && x <= 3 {
                        Note::PlayerChange { player: x }
                    } else {
                        return Err(Error::ValueError { line: line_count, field: "player change" });
                    }
                }
                Err(_) => {
                    return Err(Error::ValueError { line: line_count, field: "player change" });
                }
            };
            current_line.notes.push(note);
            continue;
        } else {
            // unknown line
            return Err(Error::ParserFailure { line: line_count });
        }
    }
    if found_end_indicator {
        Ok(lines_vec)
    } else {
        return Err(Error::MissingEndIndicator);
    }
}
