mod fs;

use std::path::PathBuf;
use std::io;

fn main() -> io::Result<()> {
    let app = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about("Server Side Includes Compiler")
        .arg(clap::Arg::with_name("indir")
            .short("i")
            .long("in")
            .value_name("DIRECTORY")
            .help("Source directory. Defaults to $PWD")
            .required(false)
            .takes_value(true))
        .arg(clap::Arg::with_name("outdir")
            .short("o")
            .long("out")
            .value_name("DIRECTORY")
            .help("Output directory. Defaults to $PWD/dist")
            .required(false)
            .takes_value(true))
        .arg(clap::Arg::with_name("follow-links")
            .short("l")
            .long("follow-links")
            .help("If given, will follow symlinks")
            .required(false)
            .takes_value(false))
    ;
    let matches = app.get_matches();

    let pwd = std::env::current_dir()?;

    let indir: PathBuf = {
        let default_in = pwd.clone();

        matches.value_of("indir")
            .map_or(default_in, |indir_val| PathBuf::from(indir_val))
    };

    let outdir: PathBuf = {
        let mut default_out = pwd.clone();
        default_out.push("dist");

        matches.value_of("outdir")
            .map_or(default_out, |outdir_val| PathBuf::from(outdir_val))
    };

    let follow_links = matches.is_present("follow-links");

    println!("Using input directory: '{}'", indir.display());
    println!("Using output directory: '{}'", outdir.display());

    let indir_scan = fs::scan(indir, follow_links);

    return Ok(());
}
