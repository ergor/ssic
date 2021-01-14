use std::path::PathBuf;
use std::io;

fn main() -> io::Result<()> {
    let app = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about("Server Side Includes Compiler")
        .arg(clap::Arg::with_name("SRCDIR")
            .help("Source directory")
            .required(true)
            .index(1))
        .arg(clap::Arg::with_name("outdir")
            .short("o")
            .long("out")
            .value_name("DIRECTORY")
            .help("Output directory. Defaults to $PWD/dist")
            .required(false)
            .takes_value(true))
    ;
    let matches = app.get_matches();

    let mut default_out = std::env::current_dir()?;
    default_out.push("dist");

    let outdir = matches.value_of("outdir")
        .map_or(default_out, |path| PathBuf::from(path));

    println!("Using output directory: '{}'", outdir.display());

    return Ok(());
}
