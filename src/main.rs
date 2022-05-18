use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
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
    write_file: Option<PathBuf>,

    /// String to search for.
    #[structopt(short, long)]
    search_string: String,
}

fn main() -> Result {
    let opt = CliOpts::from_args();
    println!("{}", opt.search_string);

    for entry in match fs::read_dir(&opt.read_dir) {
        Ok(read_dir) => read_dir,
        Err(e) => {
            eprintln!(
                "Could not read input directory at {}",
                &opt.read_dir.display()
            );
            return e;
        }
    } {}

    for letter in opt.search_string.chars() {
        println!("{}", letter);
    }

    Ok(())
}
