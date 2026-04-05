use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::fs;
use zip::ZipArchive;
use std::fs::File;
use godot::prelude::*;

pub struct AssetHandler;

impl AssetHandler {
    /// Detects the correct extension based on the first few bytes of the file content.
    pub fn get_correct_extension(content: &[u8], current_name: &str) -> String {
        let name = current_name.to_lowercase();
        
        // PNG Magic: 89 50 4E 47 0D 0A 1A 0A
        if content.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
            if !name.ends_with(".png") {
                return format!("{}.png", current_name);
            }
        }
        
        // OGG Magic: 4F 67 67 53 (OggS)
        if content.starts_with(&[0x4F, 0x67, 0x67, 0x53]) {
            if !name.ends_with(".ogg") {
                return format!("{}.ogg", current_name);
            }
        }

        // JPEG Magic: FF D8 FF
        if content.starts_with(&[0xFF, 0xD8, 0xFF]) {
            if !name.ends_with(".jpg") && !name.ends_with(".jpeg") {
                return format!("{}.jpg", current_name);
            }
        }

        current_name.to_string()
    }

    /// Extracts a file from the archive and ensures it has the correct extension.
    pub fn extract_with_extension_fix(
        archive: &mut ZipArchive<File>, 
        index: usize, 
        base_path: &Path
    ) -> Option<PathBuf> {
        let mut zip_file = match archive.by_index(index) {
            Ok(f) => f,
            Err(_) => return None,
        };

        if zip_file.is_dir() {
            return None;
        }

        // Standardize slashes and get ONLY the filename
        let zip_full_name = zip_file.name().replace('\\', "/");
        let zip_file_name = zip_full_name.split('/').last().unwrap_or(&zip_full_name);

        let mut content = Vec::new();
        if zip_file.read_to_end(&mut content).is_err() {
            return None;
        }

        let fixed_name = Self::get_correct_extension(&content, zip_file_name);
        let dest_path = base_path.join(fixed_name);

        // Ensure parent directories exist
        if let Some(parent) = dest_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let mut out_file = match File::create(&dest_path) {
            Ok(f) => f,
            Err(_) => return None,
        };

        if out_file.write_all(&content).is_err() {
            return None;
        }

        Some(dest_path)
    }

    /// Recursively fixes extensions in an existing directory.
    pub fn fix_extensions_in_dir(dir_path: &Path) {
        if !dir_path.exists() { return; }
        
        let entries = match fs::read_dir(dir_path) {
            Ok(e) => e,
            Err(_) => return,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                Self::fix_extensions_in_dir(&path);
            } else {
                let mut buffer = [0u8; 8];
                let mut f = match File::open(&path) {
                    Ok(file) => file,
                    Err(_) => continue,
                };
                
                let n = f.read(&mut buffer).unwrap_or(0);
                let content = &buffer[..n];
                
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                let fixed_name = Self::get_correct_extension(content, file_name);
                
                if fixed_name != file_name {
                    let mut new_path = path.clone();
                    new_path.set_file_name(fixed_name);
                    godot_print!("AssetHandler: Renaming {} to {:?}", file_name, new_path.file_name());
                    let _ = fs::rename(path, new_path);
                }
            }
        }
    }
}
