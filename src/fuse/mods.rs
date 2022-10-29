use nn_fuse::{FAccessor, FsAccessor, FileSystemAccessor, FsEntryType, AccessorResult, DAccessor };

pub struct ModFuse;

impl FileSystemAccessor for ModFuse {
    fn get_entry_type(&self, path: &std::path::Path) -> Result<FsEntryType, AccessorResult> {
        println!("GetEntryType: {}", path.display());
        Ok(FsEntryType::File)
    }

    fn open_file(&self, path: &std::path::Path, mode: skyline::nn::fs::OpenMode) -> Result<*mut FAccessor, AccessorResult> {
        println!("OpenFile: {}", path.display());

        Err(AccessorResult::PathNotFound)
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