use std::path::{Path, PathBuf};

#[derive(Debug)]
/// The paths of the temporary and source files.
///
/// Creates a temporary file from a source file in the test_files folder.
///
/// Drop the struct to delete the temporary file.
pub struct Tempfile
{
    temp_path: PathBuf,
    source_path: PathBuf,
}

impl Tempfile
{
    /// Create a copy of a file from test_files folder with the given name
    pub fn create_from(source_file_name: &str) -> std::io::Result<Tempfile>
    {
        let root_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest dir");
        let mut source_dir = PathBuf::from(root_dir);
        // Check if tests/test_files exists
        source_dir.push("tests");
        source_dir.push("test_files");

        println!("{:?}", source_dir);
        if source_dir.exists()
        {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Can't find /tests/test_files folder"))
        }

        let mut source_path = source_dir.to_owned();
        source_path.push(source_file_name);

        let mut temp_path = source_dir;
        temp_path.push(format!("temp_{}", source_file_name));

        Tempfile::copy(&source_path, &temp_path)?;

        Ok(Tempfile {temp_path, source_path})

    }

    /// The path to the temporary file
    pub fn temp_file(&self) -> &Path
    {
        &self.temp_path
    }

    fn copy(source: &Path, temp: &Path) -> std::io::Result<()>
    {
        match std::fs::copy(source, temp)
        {
            Ok(_) => Ok(()),
            Err(e) => {eprintln!("Can't find file, source: {:?}, temp: {:?}", source, temp); Err(e)}
        }
    }
}

impl Drop for Tempfile
{
    fn drop(&mut self)
    {
        match std::fs::remove_file(&self.temp_path)
        {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        }
    }
}