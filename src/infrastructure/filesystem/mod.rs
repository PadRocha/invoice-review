mod ensure_parent;
mod validate_paths;
mod write_text;

pub use ensure_parent::ensure_parent_directory;
pub use validate_paths::{resolve_path, validate_readable_file};
pub use write_text::write_text_file;

pub fn register_signal_handlers() {
    let _ = ctrlc::set_handler(|| {
        eprintln!("\nProceso interrumpido.");
        std::process::exit(130);
    });
}
