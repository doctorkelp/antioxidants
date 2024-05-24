use sha1::{Digest, Sha1};
use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let dir = if args.len() > 1 { &args[1] } else { "." };

    let paths = fs::read_dir(dir)?;

    for path in paths {
        let path = path?.path();
        if path.is_file() && !is_dot_file(&path) {
            let hash = compute_sha1(&path)?;
            let truncated_hash = &hash[..8]; // Use only the first 8 characters of the hash
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let new_file_name = format!("{}-{}", truncated_hash, file_name);
            let new_path = path.with_file_name(new_file_name);
            fs::rename(&path, &new_path)?;
        }
    }

    let paths = fs::read_dir(dir)?;
    let mut hash_map: std::collections::HashMap<String, Vec<std::path::PathBuf>> =
        std::collections::HashMap::new();

    for path in paths {
        let path = path?.path();
        if path.is_file() && !is_dot_file(&path) {
            if let Some(file_name) = path.file_name() {
                let file_name = file_name.to_string_lossy().to_string();
                if let Some(pos) = file_name.find('-') {
                    let hash = &file_name[..pos];
                    hash_map
                        .entry(hash.to_string())
                        .or_default()
                        .push(path.clone());
                }
            }
        }
    }

    for (hash, files) in hash_map {
        if files.len() > 1 {
            let dir_name = format!("{}/{}", dir, hash);
            fs::create_dir_all(&dir_name)?;
            for file in files.iter().skip(1) {
                let file_name = file.file_name().unwrap().to_string_lossy().to_string();
                let new_path = Path::new(&dir_name).join(&file_name);
                fs::rename(file, new_path)?;
            }
        }
    }

    Ok(())
}

fn is_dot_file(path: &Path) -> bool {
    if let Some(file_name) = path.file_name() {
        file_name.to_string_lossy().starts_with('.')
    } else {
        false
    }
}

fn compute_sha1<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha1::new();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    hasher.update(&buffer);

    Ok(format!("{:x}", hasher.finalize()))
}
