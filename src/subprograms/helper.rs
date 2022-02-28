use std::path::PathBuf;

pub fn path_exists(path: &PathBuf) -> Result<(), std::io::Error> {
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{:?} does not exist!", path)
        ));
    }
    Ok(())
}

pub fn path_does_not_exist(path: &PathBuf) -> Result<(), std::io::Error> {
    if path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("{:?} already exists!", path),
        ));
    }
    Ok(())
}