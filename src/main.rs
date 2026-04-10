fn main() {
    std::process::exit(match invrev::main_entry(std::env::args().skip(1)) {
        Ok(output) => {
            println!("{}", output);
            0
        }
        Err(error) => {
            eprintln!("{}", invrev::render_error(&error));
            error.exit_code()
        }
    });
}
