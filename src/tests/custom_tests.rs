use std::fs::File;
use std::io::Write;
use tempfile;

use crate::{get_content, get_download_dir, run_sorting};

#[test]
fn test_get_download_dir() {
    let result = get_download_dir();
    assert!(result.is_ok());
}

#[test]
fn test_get_content() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    File::create(temp_path.join("testfile.txt")).unwrap();

    let result = get_content(&temp_path);
    assert!(result.is_ok());
}

#[test]
fn test_run_sorting() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    let mut file = File::create(temp_path.join("testfile.txt")).unwrap();
    writeln!(file, "Test content").unwrap();

    let result = run_sorting();
    assert!(result.is_ok());
}
