use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use structopt::StructOpt;

use klib::errors::Result;
use klib::github::sync_repo;
use klib::notes::parse_kindle_notes;

/// default input path.
const KIND_PATH: &str = "/Volumes/Kindle/documents/My Clippings.txt";

#[derive(StructOpt, Debug)]
enum Output {
    #[structopt(about = "Output to specified directory.")]
    Path {
        /// Output directory.
        #[structopt(short, long, parse(from_os_str))]
        outdir: PathBuf,
    },
    #[structopt(about = "Upload notes to github repository.")]
    Github {
        #[structopt(short, long)]
        owner: String,
        #[structopt(short, long)]
        repo: String,
        #[structopt(short, long)]
        token: String,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(about = "the kindle notebook export tool.")]
struct Opt {
    /// Input file.
    #[structopt(short, long, parse(from_os_str), default_value = KIND_PATH)]
    input: PathBuf,

    #[structopt(subcommand)]
    output: Output,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let notes = parse_kindle_notes(opt.input)?;

    match opt.output {
        Output::Path { ref outdir } => {
            export_to_directory(outdir, &notes)?;
        }
        Output::Github {
            ref owner,
            ref repo,
            ref token,
        } => {
            sync_repo(owner, repo, token, &notes)?;
        }
    }

    Ok(())
}

fn export_to_directory(path: &PathBuf, notes: &HashMap<String, String>) -> Result<()> {
    for (name, text) in notes {
        let p = path.join(name.to_owned() + ".md");
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(p)?;

        file.write(text.as_bytes())?;
    }

    println!("kindle notes output directory: {}", path.to_str().unwrap());

    Ok(())
}
