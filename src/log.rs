use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::Path;

use chrono::Utc;

thread_local! {
    static FILE_MAP: RefCell<HashMap<String, File>> = RefCell::new(HashMap::new());
}

fn timestamp_string(data: &str) -> String {
    format!("[{}] {}", Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"), data)
}

fn write(filename: &str, data: String) -> Result<(), io::Error> {
    FILE_MAP.with(|cell| {
        let mut map = cell.borrow_mut();
        let file = match map.entry(filename.to_owned()) {
            Occupied(elem) => elem.into_mut(),
            Vacant(elem) => {
                let path = Path::new(filename);
                match path.parent() {
                    Some(p) => fs::create_dir_all(p)?,
                    None => {},
                };

                let file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(path)?;
                elem.insert(file)
            },
        };

        writeln!(file, "{}", data)
    })
}

fn close() {
    FILE_MAP.with(|cell| {
        let mut map = cell.borrow_mut();
        map.clear();
    });
}

byond_function! { log_write(filename, line) {
    let line = timestamp_string(line);

    match write(filename, line) {
        Ok(_) => None,
        Err(err) => Some(err.to_string()),
    }
} }

byond_function! { log_close_all() {
    close();

    None
} }
