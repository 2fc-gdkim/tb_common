#[derive(serde::Serialize, serde::Deserialize,  Debug, Clone)]
pub struct SMDistFile {
    pub file_path: String,
    pub dist_type: String,
    pub target_path: String,
    pub hash: String
}

impl SMDistFile {
    pub fn new() -> Self {
        Self {
            file_path: String::new(),
            dist_type: String::new(),
            target_path: String::new(),
            hash: String::new(),
        }
    }

    pub fn new_with_data(file_path: &str, dist_type: &str, target_path: &str, hash: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
            dist_type: dist_type.to_string(),
            target_path: target_path.to_string(),
            hash: hash.to_string(),
        }
    }
}