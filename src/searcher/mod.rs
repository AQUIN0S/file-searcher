use std::{
    fmt::Display,
    fs::{self, DirEntry, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub struct SearchSuccess {
    pub file_name: String,
    pub line: usize,
    pub column: usize,
}

impl Display for SearchSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Found a match in file {}, line {}, column {}!",
            self.file_name, self.line, self.column
        )
    }
}

pub fn traverse_entry(
    entry: DirEntry,
    depth_remaining: usize,
    search_str: &str,
) -> Vec<SearchSuccess> {
    let mut success_vec = Vec::new();

    let file_type = match entry.file_type() {
        Ok(t) => t,
        Err(e) => {
            return {
                eprintln!(
                    "Could not get file type for file {}",
                    entry.path().display()
                );
                eprintln!("Full error message:");
                eprintln!("{}", e);
                success_vec
            }
        }
    };

    if file_type.is_file() {
        success_vec.append(&mut search(entry.path(), search_str));
    } else if file_type.is_dir() {
        if depth_remaining <= 0 {
            eprintln!("Max depth reached at folder {}", entry.path().display());
            return success_vec;
        }

        let read_dir = match fs::read_dir(entry.path()) {
            Ok(d) => d,
            Err(e) => {
                return {
                    eprintln!("Could not read input directory: {}", entry.path().display());
                    eprintln!("Full error message:");
                    eprintln!("{}", e);
                    success_vec
                }
            }
        };

        for sub_entry in read_dir {
            // Make sure entry is valid
            let sub_entry = match sub_entry {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Could not read a file in path {}", entry.path().display());
                    eprintln!("Full error follows:");
                    eprintln!("{}", e);
                    continue;
                }
            };

            if depth_remaining > 0 {
                success_vec.append(&mut traverse_entry(
                    sub_entry,
                    depth_remaining - 1,
                    search_str,
                ));
            } else {
                eprintln!("Max depth reached at folder {}", sub_entry.path().display());
                eprintln!("Stopping here...");
            }
        }
    } else {
        eprintln!(
            "File {} is probably a symlink of some kind, which isn't exactly readable.",
            entry.path().display()
        );
    }

    success_vec
}

fn search(path: PathBuf, search_str: &str) -> Vec<SearchSuccess> {
    let mut successes = Vec::new();

    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            return {
                eprintln!("Error opening file {}", path.display());
                eprintln!("Full error:");
                eprint!("{}", e);
                successes
            }
        }
    };

    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    let mut line_count = 0;
    let mut last_iteration = false;

    loop {
        line_count += 1;

        let read_bytes = match reader.read_until(b'\n', &mut buf) {
            Ok(b) => b,
            Err(e) => {
                eprintln!(
                    "Had a problem reading line {} of file {}",
                    line_count,
                    path.display()
                );
                eprintln!("Full error: {}", e);
                0
            }
        };

        if read_bytes == 0 || buf.get(read_bytes - 1) == Some(&b'\0') {
            last_iteration = true;
        }

        if read_bytes < search_str.len() {
            continue;
        }

        let line = String::from_utf8_lossy(&buf);
        if let Some(column) = line.find(search_str) {
            let file_name = match path.file_name() {
                Some(n) => n.to_string_lossy().to_string(),
                None => {
                    eprintln!("Weird.... can't parse the file name... welp full path it is then!");
                    path.display().to_string()
                }
            };

            successes.push(SearchSuccess {
                file_name,
                line: line_count,
                column,
            })
        }
        buf.clear();

        if last_iteration {
            break;
        }
    }

    successes
}
