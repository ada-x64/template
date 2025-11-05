fn main() {
    if cfg!(feature = "dev") {
        built::write_built_file().expect("Failed to get build info");
    }
}
