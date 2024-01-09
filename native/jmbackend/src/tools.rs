use std::path::PathBuf;

pub(crate) fn join_paths(paths: Vec<&str>) -> String {
    match paths.len() {
        0 => String::default(),
        _ => {
            let mut path: PathBuf = PathBuf::new();
            for x in 0..paths.len() {
                path = path.join(paths[x]);
            }
            return path.to_str().unwrap().to_string();
        }
    }
}
