use std::collections::HashMap;

fn main() {
    tauri_bundler::DebianSettings {
        depends: None,
        files:HashMap::new(),
        // More fields are required in the 1.3.0 version of tauri-bundler
    };
}
