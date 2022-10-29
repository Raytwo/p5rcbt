use camino::Utf8PathBuf;
use nn_fuse::{FAccessor, FsAccessor, FileSystemAccessor, FsEntryType, AccessorResult, DAccessor, FileAccessor };

pub struct ModFuse;
pub struct ModFileAccessor(Utf8PathBuf);

impl FileAccessor for ModFileAccessor {
    fn read(&mut self, mut buffer: &mut [u8], offset: usize) -> Result<usize, AccessorResult> {
        Err(AccessorResult::Unimplemented)
    }

    fn get_size(&mut self) -> Result<usize, AccessorResult> {
        let size = 0;
        Ok(size)
    }
}

impl FileSystemAccessor for ModFuse {
    fn get_entry_type(&self, path: &std::path::Path) -> Result<FsEntryType, AccessorResult> {
        println!("GetEntryType: {}", path.display());
        Err(AccessorResult::PathNotFound)
    }

    fn open_file(&self, path: &std::path::Path, mode: skyline::nn::fs::OpenMode) -> Result<*mut FAccessor, AccessorResult> {
        println!("OpenFile: {}", path.display());

        let read = mode & 1 != 0;
        let write = mode >> 1 & 1 != 0;
        let append = mode >> 2 & 1 != 0;

        if write || append {
            Err(AccessorResult::Unsupported)
        } else {
            Err(AccessorResult::PathNotFound)
        }
    }

    fn open_directory(&self, path: &std::path::Path, mode: skyline::nn::fs::OpenDirectoryMode) -> Result<*mut DAccessor, AccessorResult> {
        println!("Path: {}", path.display());

        Err(AccessorResult::Unimplemented)
    }
}

pub fn install_mods_vfs() {
    let accessor = FsAccessor::new(ModFuse);
    unsafe { nn_fuse::mount("mods", &mut *accessor).unwrap() };
}