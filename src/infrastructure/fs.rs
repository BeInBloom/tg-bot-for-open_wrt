use std::path::{Path, PathBuf};

/// Trait for filesystem operations
///
/// Abstraction for testing - allows mocking file system operations
pub trait FileSystem: Send + Sync {
    fn current_exe(&self) -> std::io::Result<PathBuf>;

    fn create_dir_all(&self, path: &Path) -> std::io::Result<()>;
}

pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn current_exe(&self) -> std::io::Result<PathBuf> {
        std::env::current_exe()
    }

    fn create_dir_all(&self, path: &Path) -> std::io::Result<()> {
        std::fs::create_dir_all(path)
    }
}

impl Default for RealFileSystem {
    fn default() -> Self {
        Self
    }
}
