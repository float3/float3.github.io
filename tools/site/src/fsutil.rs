use crate::{Result, Site, SiteError};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

impl Site {
    #[cfg(feature = "photos")]
    pub(crate) fn resolve_path(&self, path: &str) -> PathBuf {
        let path = PathBuf::from(path);
        if path.is_absolute() {
            path
        } else {
            self.root.join(path)
        }
    }

    #[cfg(feature = "photos")]
    pub(crate) fn ensure_photo_input_is_private(&self, path: &Path) -> Result<()> {
        let normalized_root = normalize_path(&self.root)?;
        let normalized_path = normalize_path(path)?;
        let content_dir = normalized_root.join("content");

        if normalized_path.starts_with(&content_dir) {
            return Err(Box::new(SiteError::new(format!(
                "photo originals must not live under {}; use private/photography/originals or another untracked directory",
                content_dir.display()
            ))));
        }

        Ok(())
    }
}

pub(crate) fn find_repo_root() -> Result<PathBuf> {
    let mut current = env::current_dir()?;

    loop {
        if current.join(".git").exists()
            && (current.join("quartz.config.ts").exists()
                || current.join("quartz.config.yaml").exists()
                || current.join("quartz.ts").exists())
        {
            return Ok(current);
        }

        if !current.pop() {
            break;
        }
    }

    Err(Box::new(SiteError(
        "could not find repository root from current directory".to_string(),
    )))
}

pub(crate) fn remove_dir_if_exists(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    Ok(())
}

pub(crate) fn remove_file_if_exists(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub(crate) fn remove_license_files(path: &Path) -> Result<()> {
    if !path.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with("LICENSE.txt") {
            fs::remove_file(entry.path())?;
        }
    }

    Ok(())
}

#[cfg(feature = "photos")]
fn normalize_path(path: &Path) -> Result<PathBuf> {
    if path.exists() {
        Ok(path.canonicalize()?)
    } else if let Some(parent) = path.parent() {
        let parent = normalize_path(parent)?;
        Ok(parent.join(
            path.file_name()
                .ok_or_else(|| SiteError::new(format!("invalid path: {}", path.display())))?,
        ))
    } else {
        Ok(path.to_path_buf())
    }
}
