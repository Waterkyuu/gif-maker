use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// Scans a directory and returns sorted image paths.
///
/// Input example when `dir` is `"frames"`:
///
/// ```text
/// frames/·
///   001.png
///   002.jpg
///   readme.txt
/// ```
///
/// Output example:
///
/// ```text
/// [
///   "frames/001.png",
///   "frames/002.jpg",
/// ]
/// ```
pub fn scan_images(dir: &str) -> Result<Vec<PathBuf>> {
    let mut files = vec![];

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();


        // The order of GIF animations depends on the file sorting, so it is best to name them as
        // frame_001.png frame_002.png
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();

            if matches!(ext.as_str(), "png" | "jpg" | "jpeg") {
                files.push(path);
            }
        }
    }

    files.sort();

    Ok(files)
}
