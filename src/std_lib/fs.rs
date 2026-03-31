//! Filesystem operations.
//!
//! Demonstrates common methods from `std::fs` and `std::path`:
//! file reading, writing, metadata, and directory operations.

use std::fs;
use std::path::Path;

/// Write string to file.
///
/// `fs::write()` creates or overwrites a file.
pub fn write_file() {
    let temp_path = "test_temp_write.txt";
    fs::write(temp_path, "Hello, World!").unwrap();

    let content = fs::read_to_string(temp_path).unwrap();
    assert_eq!(content, "Hello, World!");

    fs::remove_file(temp_path).unwrap();
}

/// Read file to string.
///
/// `fs::read_to_string()` reads entire file into a String.
pub fn read_to_string_file() {
    let temp_path = "test_temp_read.txt";
    fs::write(temp_path, "Hello, World!").unwrap();

    let content = fs::read_to_string(temp_path).unwrap();
    assert_eq!(content, "Hello, World!");

    fs::remove_file(temp_path).unwrap();
}

/// Read file to bytes.
///
/// `fs::read()` reads entire file into a `Vec<u8>`.
pub fn read_to_bytes() {
    let temp_path = "test_temp_bytes.txt";
    fs::write(temp_path, "Hello").unwrap();

    let bytes = fs::read(temp_path).unwrap();
    assert_eq!(bytes, b"Hello");

    fs::remove_file(temp_path).unwrap();
}

/// Get file metadata.
///
/// `fs::metadata()` returns file information.
pub fn file_metadata() {
    let temp_path = "test_temp_meta.txt";
    fs::write(temp_path, "Hello").unwrap();

    let metadata = fs::metadata(temp_path).unwrap();
    assert!(metadata.is_file());
    assert!(!metadata.is_dir());

    fs::remove_file(temp_path).unwrap();
}

/// Check if path exists.
///
/// `Path::exists()` checks if a path exists.
pub fn path_exists() {
    let temp_path = "test_temp_exists.txt";
    fs::write(temp_path, "Hello").unwrap();

    assert!(Path::new(temp_path).exists());
    assert!(!Path::new("/path/that/does/not/exist").exists());

    fs::remove_file(temp_path).unwrap();
}

/// Create directories recursively.
///
/// `fs::create_dir_all()` creates all directories
/// in the path if they don't exist.
pub fn create_dirs() {
    fs::create_dir_all("test_dir/sub_dir").unwrap();
    assert!(Path::new("test_dir/sub_dir").exists());
    fs::remove_dir_all("test_dir").unwrap();
}

/// Read directory entries.
///
/// `fs::read_dir()` returns an iterator over directory entries.
pub fn read_directory() {
    let count = fs::read_dir(".")
        .unwrap()
        .filter_map(|e| e.ok())
        .count();
    assert!(count > 0);
}

/// Copy file to new location.
///
/// `fs::copy()` copies a file, returns bytes copied.
pub fn copy_file() {
    let src = "test_temp_src.txt";
    let dst = "test_temp_dst.txt";

    fs::write(src, "Hello").unwrap();
    let bytes = fs::copy(src, dst).unwrap();
    assert_eq!(bytes, 5);

    fs::remove_file(src).unwrap();
    fs::remove_file(dst).unwrap();
}

/// Remove file.
///
/// `fs::remove_file()` deletes a file.
pub fn remove_file_op() {
    let temp_path = "test_temp_remove.txt";
    fs::write(temp_path, "Hello").unwrap();
    fs::remove_file(temp_path).unwrap();
    assert!(!Path::new(temp_path).exists());
}

/// Remove directory recursively.
///
/// `fs::remove_dir_all()` deletes a directory
/// and all its contents.
pub fn remove_dir_all_op() {
    fs::create_dir_all("test_dir/sub_dir").unwrap();
    fs::remove_dir_all("test_dir").unwrap();
    assert!(!Path::new("test_dir").exists());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_file() {
        write_file();
    }

    #[test]
    fn test_read_to_string_file() {
        read_to_string_file();
    }

    #[test]
    fn test_read_to_bytes() {
        read_to_bytes();
    }

    #[test]
    fn test_file_metadata() {
        file_metadata();
    }

    #[test]
    fn test_path_exists() {
        path_exists();
    }

    #[test]
    fn test_create_dirs() {
        create_dirs();
    }

    #[test]
    fn test_read_directory() {
        read_directory();
    }

    #[test]
    fn test_copy_file() {
        copy_file();
    }

    #[test]
    fn test_remove_file_op() {
        remove_file_op();
    }

    #[test]
    fn test_remove_dir_all_op() {
        remove_dir_all_op();
    }
}
