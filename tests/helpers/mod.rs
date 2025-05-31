use tempfile::TempDir;
use std::fs;
use std::path::PathBuf;

pub struct TestEnvironment {
    pub temp_dir: TempDir,
    pub helmfile_path: PathBuf,
    pub config_path: PathBuf,
}

impl TestEnvironment {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let helmfile_path = temp_dir.path().join("helmfile.yaml");
        let config_path = temp_dir.path().join("helmctl.yaml");

        Self {
            temp_dir,
            helmfile_path,
            config_path,
        }
    }

    pub fn create_valid_helmfile(&self) {
        let content = include_str!("../fixtures/helmfiles/valid-helmfile.yaml");
        fs::write(&self.helmfile_path, content).unwrap();
    }

    pub fn create_invalid_helmfile(&self) {
        let content = include_str!("../fixtures/helmfiles/invalid-helmfile.yaml");
        fs::write(&self.helmfile_path, content).unwrap();
    }

    pub fn create_config(&self) {
        let content = include_str!("../fixtures/configs/valid-config.yaml");
        fs::write(&self.config_path, content).unwrap();
    }

    pub fn helmfile_path_str(&self) -> &str {
        self.helmfile_path.to_str().unwrap()
    }

    pub fn config_path_str(&self) -> &str {
        self.config_path.to_str().unwrap()
    }
}

#[macro_export]
macro_rules! assert_contains {
    ($haystack:expr, $needle:expr) => {
        assert!(
            $haystack.contains($needle),
            "Expected '{}' to contain '{}'",
            $haystack,
            $needle
        )
    };
}

#[macro_export]
macro_rules! assert_not_contains {
    ($haystack:expr, $needle:expr) => {
        assert!(
            !$haystack.contains($needle),
            "Expected '{}' to not contain '{}'",
            $haystack,
            $needle
        )
    };
}
