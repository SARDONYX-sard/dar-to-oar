use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

/// Forcibly writes content to a given path (file). If the file does not exist, create it.
pub(super) fn write_file_with_force<P>(path: P, content: &str) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub(super) fn copy_files<P>(source_dir: P, dest_dir: &Path) -> io::Result<()>
where
    P: AsRef<Path>,
{
    // Create the destination directory if it doesn't exist
    fs::create_dir_all(dest_dir)?;

    // Iterate over the entries in the source directory
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let source_path = entry.path();

        if source_path.is_file() {
            let file_name = source_path.file_name().unwrap();
            let dest_path = dest_dir.join(file_name);
            fs::copy(&source_path, &dest_path)?;
            log::trace!("Copied: {:?}", dest_path);
        }
    }

    Ok(())
}
