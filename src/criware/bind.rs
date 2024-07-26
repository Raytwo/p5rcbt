use thiserror::Error;
use camino::{Utf8Path, Utf8PathBuf};

#[derive(Debug, Error)]
pub enum DirBindingError {
    #[error("failed to bind directory with result: {0}")]
    CriwareErrorCode(i32),
    #[error("the path provided is not absolute: {0}")]
    PathNotAbsolute(Utf8PathBuf),
}

pub type CriFnBinderHandle = u64;

#[skyline::from_offset(0x8a92d0)]
pub fn get_binder_handle() -> CriFnBinderHandle;

// Old offset: 0x1302710
#[skyline::from_offset(0x1302150)]
pub fn crifsbinder_bind_directory(binder: CriFnBinderHandle, src_binder: *const u8, path: *const u8, work: *const u8, work_size: i32, bind_id: &mut u32) -> i32;

pub fn bind_directory<P: AsRef<Utf8Path>>(binder_handle: CriFnBinderHandle, path: P) -> Result<u32, DirBindingError> {
    let path = path.as_ref();

    // nn::fs::CheckMountName?

    if !path.is_absolute() {
        Err(DirBindingError::PathNotAbsolute(path.to_path_buf()))
    } else {
        let nullterm_path = format!("{}\0", path);

        let bind_path = skyline::c_str(&nullterm_path);
        let mut out_bind_id = 0;

        let result = unsafe { crifsbinder_bind_directory(binder_handle, 0 as _, bind_path, 0 as _, 0, &mut out_bind_id) };

        if result != 0 {
            Err(DirBindingError::CriwareErrorCode(result))
        } else {
            Ok(out_bind_id)
        }
    }
}