use std::collections::HashMap;
use std::fs;
use std::ops::Range;
use std::path::Path;

use regex::Regex;

use crate::errors::{Error, Result};

lazy_static! {
    static ref RE: Regex = Regex::new("(\\d+)-(\\d+)").unwrap();
}

/// the clippings note item.
#[derive(Debug)]
struct Note {
    // sign position, format #start-end.
    pos: Range<i32>,
    // mark text.
    pub text: String,
}

/// parse kindle notes.
///
/// kindle note format:
///  book name\n
///  mark position\n
///  \n
///  note text\n
///  ==========\n
pub fn parse_kindle_notes<P: AsRef<Path>>(path: P) -> Result<HashMap<String, String>> {
    let mut kmap: HashMap<String, Vec<Note>> = HashMap::new();
    let mut stem = Vec::new();

    let pbuf = path.as_ref().to_path_buf();
    let contents = fs::read_to_string(path).map_err(|e| Error::File(pbuf, e))?;

    for line in contents.lines() {
        stem.push(line.to_string());

        if stem.len() >= 5 {
            let (name, note) = parse_note(&stem)?;

            match kmap.get_mut(&name) {
                Some(array) => {
                    // repeat mark position, use last record.
                    match array.iter().position(|x| x.pos.start == note.pos.start) {
                        Some(n) => {
                            array[n] = note;
                        }
                        None => {
                            array.push(note);
                        }
                    }
                }
                None => {
                    kmap.insert(name, vec![note]);
                }
            }
            stem.clear();
        }
    }

    // convert into HashMap<String, String>
    flat_notes(&kmap)
}

fn parse_note(stem: &Vec<String>) -> Result<(String, Note)> {
    // parse mark range.
    let r = stem[1].to_string();
    let pos = try_parse_range(&r).ok_or(Error::NoteRange(r))?;

    let name = stem[0].to_string();
    let text = stem[3].to_string();

    Ok((name, Note { pos, text }))
}

// parse mark position [start]-[end]
fn try_parse_range(s: &str) -> Option<Range<i32>> {
    let caps = RE.captures(s)?;
    let start = caps.get(1)?.as_str().parse::<i32>().ok()?;
    let end = caps.get(2)?.as_str().parse::<i32>().ok()?;

    Some(Range { start, end })
}

fn flat_notes(kmap: &HashMap<String, Vec<Note>>) -> Result<HashMap<String, String>> {
    let mut res = HashMap::new();

    for (name, notes) in kmap {
        let mut s = String::new();
        for note in notes {
            s.push_str("- ");
            s.push_str(&note.text);
            s.push_str("\n\n");
        }
        res.insert(name.clone(), s);
    }

    Ok(res)
}
