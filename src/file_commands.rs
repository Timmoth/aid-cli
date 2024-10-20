use md5::{Digest as Md5Digest, Md5};
use sha1::Sha1;
use sha2::Sha256;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::time::UNIX_EPOCH;
use time::OffsetDateTime;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn file_info(file_path: String) {
    let path = Path::new(&file_path);

    // Check if file exists
    if !path.exists() {
        println!("File does not exist.");
        return;
    }

    // Get metadata
    let metadata = path.metadata().expect("Failed reading file metadata");

    // File type: Directory or file
    if metadata.is_dir() {
        println!("Type: Directory");
    } else if metadata.is_file() {
        println!("Type: File");
    } else if metadata.file_type().is_symlink() {
        println!("Type: Symlink");
    } else {
        println!("Type: Other");
    }

    println!("Size: {} bytes", metadata.len());

    // Permissions (cross-platform, but more detailed on Unix)
    #[cfg(unix)]
    println!("Permissions: {:o}", metadata.permissions().mode());
    #[cfg(not(unix))]
    println!(
        "Permissions: Read-only: {}",
        metadata.permissions().readonly()
    );

    // Timestamps
    let modified_time = metadata
        .modified()
        .expect("Failed reading date modified")
        .duration_since(UNIX_EPOCH)
        .expect("Failed reading date modified")
        .as_secs();
    let accessed_time = metadata
        .accessed()
        .expect("Failed reading date accessed.")
        .duration_since(UNIX_EPOCH)
        .expect("Failed reading date accessed.")
        .as_secs();

    // On some platforms, creation time may not be available
    let created_time = metadata.created().ok().map_or_else(
        || "Not available".to_string(),
        |created| {
            let created_secs = OffsetDateTime::from_unix_timestamp(created.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64).expect("Failed converting date accessed.");
            format!("{}", created_secs)
        },
    );

    let modified = OffsetDateTime::from_unix_timestamp(modified_time as i64).expect("Failed converting date modified.");
    let accessed = OffsetDateTime::from_unix_timestamp(accessed_time as i64).expect("Failed converting date accessed.");

    println!("Modified: {}", modified);
    println!("Accessed: {}", accessed);
    println!("Created: {}", created_time);
}

pub fn md5_checksum(file_path: String) {
    let file = File::open(&file_path).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut md5_hasher = Md5::new();
    let mut buffer = [0; 1024]; // 1KB buffer

    loop {
        let bytes_read = reader.read(&mut buffer).expect("Failed reading file data");

        if bytes_read == 0 {
            break; // EOF reached
        }

        md5_hasher.update(&buffer[..bytes_read]);
    }

    let md5_result = md5_hasher.finalize();
    println!("MD5: {:x}", md5_result);
}

pub fn sha1_checksum(file_path: String) {
    let file = File::open(&file_path).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut sha1_hasher = Sha1::new();
    let mut buffer = [0; 1024]; // 1KB buffer

    loop {
        let bytes_read = reader.read(&mut buffer).expect("Failed reading file data");

        if bytes_read == 0 {
            break; // EOF reached
        }

        sha1_hasher.update(&buffer[..bytes_read]);
    }

    let sha1_result = sha1_hasher.finalize();
    println!("SHA-1: {:x}", sha1_result);
}

pub fn sha256_checksum(file_path: String) {
    let file = File::open(&file_path).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut sha256_hasher = Sha256::new();
    let mut buffer = [0; 1024]; // 1KB buffer

    loop {
        let bytes_read = reader.read(&mut buffer).expect("Failed reading file data");

        if bytes_read == 0 {
            break; // EOF reached
        }

        sha256_hasher.update(&buffer[..bytes_read]);
    }

    let sha256_result = sha256_hasher.finalize();
    println!("SHA-256: {:x}", sha256_result);
}