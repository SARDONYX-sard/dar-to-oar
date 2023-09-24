use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use crate::converter::fs::write_configs;

pub(super) fn read_file<P>(file_path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = fs::File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Traverse the directory recursively to find _conditions.txt.
/// - call_back(full_path): NOTE: This is function ptr. **non closure**
pub(super) fn visit_dirs<P>(
    dir: P,
    call_back: Option<fn(PathBuf) -> io::Result<()>>,
) -> io::Result<()>
where
    P: AsRef<Path>,
{
    if dir.as_ref().is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, call_back.or(None))?;
            } else if path.extension().is_some() {
                log::trace!("File: {:?}", path);
                if let Some(func) = call_back {
                    func(path)?
                }
            }
        }
    }
    Ok(())
}

fn generate_configs(path: PathBuf) -> io::Result<()> {
    if let Some(extension) = path.extension() {
        if extension == "txt" && path.file_name() == Some(std::ffi::OsStr::new("_conditions.txt")) {
            match read_file(&path) {
                Ok(content) => {
                    log::trace!("Content:\n{}", content);
                    write_configs(path, &content)?
                }
                Err(err) => eprintln!("Error reading file {:?}: {}", path, err),
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::converter::fs::visiter::visit_dirs;
    use std::path::Path;

    // use super::generate_configs;
    // #[test]
    // fn test_generate() {
    //     let start_dir = Path::new("../test/Smooth Moveset");
    //     if let Err(err) = visit_dirs(
    //         &start_dir,
    //         Some(generate_configs),
    //     ) {
    //         eprintln!("Error: {}", err);
    //     }
    // }

    #[test]
    fn should_traverse() {
        let start_dir = Path::new("../test/Smooth Moveset");
        if let Err(err) = visit_dirs(
            &start_dir,
            Some(|dir| {
                println!("{}", dir.to_string_lossy());
                Ok(())
            }),
        ) {
            eprintln!("Error: {}", err);
        }
    }
}
