mod input;

fn main() {
    let arg_matches = clap::App::new("s502-as 0.1")
        .arg(
            clap::Arg::with_name("assemble only")
                .short("a")
                .long("assemble")
                .help("Only assemble, do not link"),
        )
        .arg(
            clap::Arg::with_name("output symbol tables")
                .short("s")
                .long("symbols")
                .help("Output a symbol table for each source file"),
        )
        .arg(
            clap::Arg::with_name("output combined symbol table")
                .short("c")
                .long("combined-symbols")
                .help("Output a single symbol table of all source files combined"),
        )
        .arg(
            clap::Arg::with_name("sources")
                .multiple(true)
                .required(true)
                .help("The source code file names (*.65a)"),
        )
        .get_matches();

    // let compile_only = arg_matches.is_present("compile only");

    let _source_files = arg_matches.values_of_lossy("sources").unwrap();
}