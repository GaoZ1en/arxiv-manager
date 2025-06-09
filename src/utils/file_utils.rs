use super::{UtilError, UtilResult};
use anyhow::Context;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs as async_fs;

/// File utility functions
pub struct FileUtils;

impl FileUtils {
    /// Ensure a directory exists, creating it if necessary
    pub fn ensure_dir_exists<P: AsRef<Path>>(path: P) -> UtilResult<()> {
        let path = path.as_ref();
        if !path.exists() {
            fs::create_dir_all(path)
                .map_err(|e| UtilError::FileError(format!("Failed to create directory {}: {}", path.display(), e)))?;
        }
        Ok(())
    }
    
    /// Async version of ensure_dir_exists
    pub async fn ensure_dir_exists_async<P: AsRef<Path>>(path: P) -> UtilResult<()> {
        let path = path.as_ref();
        if !path.exists() {
            async_fs::create_dir_all(path)
                .await
                .map_err(|e| UtilError::FileError(format!("Failed to create directory {}: {}", path.display(), e)))?;
        }
        Ok(())
    }
    
    /// Get file size in bytes
    pub fn get_file_size<P: AsRef<Path>>(path: P) -> UtilResult<u64> {
        let metadata = fs::metadata(path.as_ref())
            .map_err(|e| UtilError::FileError(format!("Failed to get file metadata: {}", e)))?;
        Ok(metadata.len())
    }
    
    /// Check if file exists and is readable
    pub fn is_readable<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists() && path.as_ref().is_file()
    }
    
    /// Get file extension
    pub fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
    }
    
    /// Sanitize filename by removing invalid characters
    pub fn sanitize_filename(filename: &str) -> String {
        filename
            .chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                c if c.is_control() => '_',
                c => c,
            })
            .collect::<String>()
            .trim()
            .to_string()
    }
    
    /// Generate unique filename if file already exists
    pub fn get_unique_filename<P: AsRef<Path>>(path: P) -> PathBuf {
        let path = path.as_ref();
        
        if !path.exists() {
            return path.to_path_buf();
        }
        
        let parent = path.parent().unwrap_or(Path::new("."));
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        
        for i in 1..1000 {
            let new_name = if extension.is_empty() {
                format!("{}_{}", stem, i)
            } else {
                format!("{}_{}.{}", stem, i, extension)
            };
            
            let new_path = parent.join(new_name);
            if !new_path.exists() {
                return new_path;
            }
        }
        
        // Fallback if we can't find a unique name
        path.to_path_buf()
    }
    
    /// Copy file with progress callback
    pub async fn copy_file_with_progress<P1, P2, F>(
        from: P1,
        to: P2,
        mut progress_callback: F,
    ) -> UtilResult<()>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
        F: FnMut(u64, u64),
    {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        
        let from_path = from.as_ref();
        let to_path = to.as_ref();
        
        let mut source = async_fs::File::open(from_path)
            .await
            .map_err(|e| UtilError::FileError(format!("Failed to open source file: {}", e)))?;
        
        let mut dest = async_fs::File::create(to_path)
            .await
            .map_err(|e| UtilError::FileError(format!("Failed to create destination file: {}", e)))?;
        
        let total_size = source.metadata()
            .await
            .map_err(|e| UtilError::FileError(format!("Failed to get file metadata: {}", e)))?
            .len();
        
        let mut buffer = vec![0u8; 8192];
        let mut copied = 0u64;
        
        loop {
            let bytes_read = source.read(&mut buffer)
                .await
                .map_err(|e| UtilError::FileError(format!("Failed to read from source: {}", e)))?;
            
            if bytes_read == 0 {
                break;
            }
            
            dest.write_all(&buffer[..bytes_read])
                .await
                .map_err(|e| UtilError::FileError(format!("Failed to write to destination: {}", e)))?;
            
            copied += bytes_read as u64;
            progress_callback(copied, total_size);
        }
        
        dest.flush()
            .await
            .map_err(|e| UtilError::FileError(format!("Failed to flush destination: {}", e)))?;
        
        Ok(())
    }
    
    /// Get all files in directory with specific extension
    pub fn get_files_with_extension<P: AsRef<Path>>(
        dir: P,
        extension: &str,
        recursive: bool,
    ) -> UtilResult<Vec<PathBuf>> {
        let mut files = Vec::new();
        let extension = extension.to_lowercase();
        
        fn visit_dir(
            dir: &Path,
            extension: &str,
            recursive: bool,
            files: &mut Vec<PathBuf>,
        ) -> io::Result<()> {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext.to_string_lossy().to_lowercase() == extension {
                            files.push(path);
                        }
                    }
                } else if path.is_dir() && recursive {
                    visit_dir(&path, extension, recursive, files)?;
                }
            }
            Ok(())
        }
        
        visit_dir(dir.as_ref(), &extension, recursive, &mut files)
            .map_err(|e| UtilError::FileError(format!("Failed to read directory: {}", e)))?;
        
        Ok(files)
    }
    
    /// Format file size as human-readable string
    pub fn format_file_size(size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        const THRESHOLD: f64 = 1024.0;
        
        if size == 0 {
            return "0 B".to_string();
        }
        
        let mut size = size as f64;
        let mut unit_index = 0;
        
        while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
            size /= THRESHOLD;
            unit_index += 1;
        }
        
        if unit_index == 0 {
            format!("{} {}", size as u64, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }
    
    /// Check if path is safe (no directory traversal)
    pub fn is_safe_path<P: AsRef<Path>>(path: P) -> bool {
        let path = path.as_ref();
        
        // Check for directory traversal patterns
        let path_str = path.to_string_lossy();
        if path_str.contains("..") || path_str.contains("~") {
            return false;
        }
        
        // Check if path is absolute when it shouldn't be
        if path.is_absolute() {
            return false;
        }
        
        true
    }
    
    /// Get temporary file path
    pub fn get_temp_file_path(prefix: &str, extension: &str) -> PathBuf {
        let filename = format!("{}-{}.{}", prefix, uuid::Uuid::new_v4(), extension);
        std::env::temp_dir().join(filename)
    }
    
    /// Clean up old temporary files
    pub fn cleanup_temp_files(prefix: &str, max_age_hours: u64) -> UtilResult<usize> {
        let temp_dir = std::env::temp_dir();
        let mut cleaned = 0;
        
        let cutoff_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - (max_age_hours * 3600);
        
        for entry in fs::read_dir(&temp_dir)
            .map_err(|e| UtilError::FileError(format!("Failed to read temp directory: {}", e)))?
        {
            let entry = entry
                .map_err(|e| UtilError::FileError(format!("Failed to read directory entry: {}", e)))?;
            
            let path = entry.path();
            let filename = path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            
            if filename.starts_with(prefix) {
                if let Ok(metadata) = fs::metadata(&path) {
                    if let Ok(modified) = metadata.modified() {
                        if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                            if duration.as_secs() < cutoff_time {
                                if fs::remove_file(&path).is_ok() {
                                    cleaned += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(cleaned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{TempDir, NamedTempFile};
    use std::io::Write;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(FileUtils::sanitize_filename("file<>name.pdf"), "file__name.pdf");
        assert_eq!(FileUtils::sanitize_filename("normal_file.pdf"), "normal_file.pdf");
        assert_eq!(FileUtils::sanitize_filename("file/with\\path.pdf"), "file_with_path.pdf");
    }

    #[test]
    fn test_get_extension() {
        assert_eq!(FileUtils::get_extension("file.pdf"), Some("pdf".to_string()));
        assert_eq!(FileUtils::get_extension("file.PDF"), Some("pdf".to_string()));
        assert_eq!(FileUtils::get_extension("file"), None);
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(FileUtils::format_file_size(0), "0 B");
        assert_eq!(FileUtils::format_file_size(512), "512 B");
        assert_eq!(FileUtils::format_file_size(1024), "1.0 KB");
        assert_eq!(FileUtils::format_file_size(1536), "1.5 KB");
        assert_eq!(FileUtils::format_file_size(1048576), "1.0 MB");
    }

    #[test]
    fn test_is_safe_path() {
        assert!(FileUtils::is_safe_path("safe/path"));
        assert!(!FileUtils::is_safe_path("../unsafe/path"));
        assert!(!FileUtils::is_safe_path("~/home/path"));
        assert!(!FileUtils::is_safe_path("/absolute/path"));
    }

    #[test]
    fn test_ensure_dir_exists() {
        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path().join("test_subdir");
        
        assert!(!test_dir.exists());
        FileUtils::ensure_dir_exists(&test_dir).unwrap();
        assert!(test_dir.exists());
    }

    #[test]
    fn test_get_unique_filename() {
        let temp_dir = TempDir::new().unwrap();
        let original = temp_dir.path().join("test.txt");
        
        // Create the original file
        std::fs::write(&original, "test").unwrap();
        
        let unique = FileUtils::get_unique_filename(&original);
        assert_ne!(unique, original);
        assert!(unique.to_string_lossy().contains("test_1.txt"));
    }

    #[tokio::test]
    async fn test_copy_file_with_progress() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("source.txt");
        let dest = temp_dir.path().join("dest.txt");
        
        // Create source file
        std::fs::write(&source, "test content").unwrap();
        
        let mut progress_calls = 0;
        FileUtils::copy_file_with_progress(&source, &dest, |copied, total| {
            progress_calls += 1;
            assert!(copied <= total);
        }).await.unwrap();
        
        assert!(dest.exists());
        assert!(progress_calls > 0);
        assert_eq!(std::fs::read_to_string(&dest).unwrap(), "test content");
    }
}
