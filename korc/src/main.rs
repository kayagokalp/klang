fn main() {
    if let Err(err) = korc::cli::run_cli() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
