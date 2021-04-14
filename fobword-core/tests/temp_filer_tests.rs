use fobword_core::temp_filer::Tempfile;
use std::{io::ErrorKind, path::PathBuf};

#[test]
fn file_is_created_and_deleted()
{
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest dir");
    let mut temp_file = PathBuf::from(root_dir);
    temp_file.push("tests");
    temp_file.push("test_files");
    temp_file.push("temp_test");

    {
        let file = Tempfile::create_from("test");
        assert!(temp_file.exists());
    }
    assert!(!temp_file.exists());
}

#[test]
fn file_does_not_exist()
{
    let result = Tempfile::create_from("Some random file name that shouldnt exist").map_err(|e|e.kind());
    let expected: Result<Tempfile, ErrorKind> = Err(std::io::ErrorKind::NotFound);
    assert_eq!(expected.err(), result.err());
}