#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate lazy_static;

use std::fs::OpenOptions;
use std::io::IoSlice;
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
    let kns = parse_kindle_notes(opt.input)?;

    for (name, notes) in kns {
        let p = opt.outdir.join(name + ".md");
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .write(true)
            .open(p)?;

        for note in notes {
            file.write_vectored(&vec![
                IoSlice::new(b"- "),
                IoSlice::new(note.text.as_bytes()),
                IoSlice::new(b"\n\n"),
            ])?;
        }
    }

    println!("output directory: {}", opt.outdir.to_str().unwrap());

    Ok(())
}
