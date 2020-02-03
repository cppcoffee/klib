#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate lazy_static;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use structopt::StructOpt;

mod errors;
use errors::Result;

mod notes;
use notes::parse_kindle_notes;

/// default input path.
const KIND_PATH: &str = "/Volumes/Kindle/documents/My Clippings.txt";

#[derive(StructOpt, Debug)]
#[structopt(about = "the kindle notebook export tool.")]
struct Opt {
    /// Input file.
    #[structopt(short, long, parse(from_os_str), default_value = KIND_PATH)]
    input: PathBuf,

    /// Output directory.
    #[structopt(short, long, parse(from_os_str))]
    outdir: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let notes = parse_kindle_notes(opt.input)?;

    for (name, text) in notes {
        let p = opt.outdir.join(name + ".md");
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .write(true)
            .open(p)?;

        file.write(text.as_bytes())?;
    }

    println!("output directory: {}", opt.outdir.to_str().unwrap());

    Ok(())
}
