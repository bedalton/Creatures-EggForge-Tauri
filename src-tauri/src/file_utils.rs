use std::path::PathBuf;

pub(crate) fn is_creatures_file(path: PathBuf) -> bool {
    match path.extension().unwrap().to_str() {
        Some(ext) => {
            match ext.to_lowercase().as_str() {
                "att" => true,
                "c16" => true,
                "gno" => true,
                "gen" => true,
                "agent" => true,
                "agents" => true,
                _ => false
            }
        }
        None => false
    }
}