mod searcher;

use std::{fs, io, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "File Searcher",
    about = "Recursively search through your files."
)]
struct CliOpts {
    /// Input directory.
    #[structopt(short, long, parse(from_os_str))]
    read_dir: PathBuf,

    /// File name to write the output to, in case you don't want to have to trawl
    /// through the console output. Two files will be generated:
    #[structopt(short, long, parse(from_os_str))]
    _write_file: Option<PathBuf>,

    /// String to search for.
    #[structopt(short, long)]
    search_string: String,

    /// Max depth to search through
    #[structopt(short, long, default_value = "10")]
    depth: usize,
}

fn main() -> io::Result<()> {
    let opt = CliOpts::from_args();
    println!("Reading from directory {}", opt.read_dir.display());
    println!("Searching for '{}'", opt.search_string);

    let read_dir = match fs::read_dir(&opt.read_dir) {
        Ok(d) => d,
        Err(e) => {
            return {
                eprintln!(
                    "Could not read input directory: {}",
                    &opt.read_dir.display()
                );
                println!("Exiting program...");
                Err(e)
            }
        }
    };

    let mut found_entries = Vec::new();

    for entry in read_dir {
        // Make sure entry is valid
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Could not read a file in path {}", &opt.read_dir.display());
                eprintln!("Full error follows:");
                eprintln!("{}", e);
                continue;
            }
        };

        found_entries.append(&mut searcher::traverse_entry(
            entry,
            opt.depth,
            &opt.search_string,
        ));
    }

    for entry in found_entries {
        println!("{}", entry);
    }

    Ok(())
}
