use crate::imp::prelude::*;
use std::{env, fs, io};
use zip::{result::ZipError, ZipArchive};

#[derive(Debug, Clone, PartialEq)]
pub struct Driver {
    path: PathBuf,
}

impl Driver {
    const ZIP: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), env!("SEP"), "driver.zip"));
    const PLATFORM: &'static str = include_str!(concat!(env!("OUT_DIR"), env!("SEP"), "platform"));

    pub fn install() -> io::Result<Self> {
        let path = Self::default_dest();
        println!("Path for driver install() {:?}", path);

        let this = Self::new(path);
        if !this.path.is_dir() {
            this.prepare()?;
        }
        Ok(this)
    }

    /**
     * This is a new method that allows us to set the path outselves
     */
    pub fn install_to_path(path: PathBuf) -> io::Result<Self> {
        let this = Self::new(&path);
        println!("Path for driver install_to_path() {:?}", path);

        println!(
            "Readable and is empty: {:?}",
            this.path
                .read_dir()
                .map(|mut i| i.next().is_none())
                .unwrap_or(false)
        );

        if !this.path.is_dir() {
            this.prepare()?;
        }

        Ok(this)
    }

    /// Without prepare
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }
    ///
    pub fn prepare(&self) -> Result<(), ZipError> {
        println!("Preparing installation");
        fs::create_dir_all(&self.path)?;
        println!("Installation prepared");
        let mut a = ZipArchive::new(io::Cursor::new(Self::ZIP))?;
        println!("Getting Archive");
        let result = a.extract(&self.path);
        println!("Archive extracted");
        return result;
    }

    pub fn default_dest() -> PathBuf {
        let base: PathBuf = dirs::cache_dir().unwrap_or_else(env::temp_dir);
        let dir: PathBuf = [
            base.as_os_str(),
            "ms-playwright".as_ref(),
            "playwright-rust".as_ref(),
            "driver".as_ref(),
        ]
        .iter()
        .collect();
        dir
    }

    pub fn platform(&self) -> Platform {
        match Self::PLATFORM {
            "linux" => Platform::Linux,
            "mac" => Platform::Mac,
            "win32" => Platform::Win32,
            "win32_x64" => Platform::Win32x64,
            _ => unreachable!(),
        }
    }

    pub fn executable(&self) -> PathBuf {
        let executable_path = match env::var("EXECUTABLE_SCRIPT") {
            Ok(path) => PathBuf::from(path),
            _ => match self.platform() {
                Platform::Linux => self.path.join("playwright.sh"),
                Platform::Mac => self.path.join("playwright.sh"),
                Platform::Win32 => self.path.join("playwright.cmd"),
                Platform::Win32x64 => self.path.join("playwright.cmd"),
            },
        };

        println!("Executable path {:?}", executable_path);

        return executable_path;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Linux,
    Win32,
    Win32x64,
    Mac,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install() {
        let _driver = Driver::install().unwrap();
    }
}
