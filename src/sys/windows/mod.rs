pub(crate) mod fdentry_impl;
pub(crate) mod host_impl;
pub(crate) mod hostcalls_impl;

use crate::sys::errno_from_ioerror;
use crate::Result;
use std::fs::File;
use std::path::Path;

pub(crate) fn dev_null() -> Result<File> {
    File::open("NUL").map_err(errno_from_ioerror)
}

pub fn preopen_dir<P: AsRef<Path>>(path: P) -> Result<File> {
    use std::fs::OpenOptions;
    use std::os::windows::fs::OpenOptionsExt;
    use winapi::um::winbase::FILE_FLAG_BACKUP_SEMANTICS;

    // To open a directory using CreateFile, specify the
    // FILE_FLAG_BACKUP_SEMANTICS flag as part of dwFileFlags...
    // cf. https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-createfile2
    OpenOptions::new()
        .create(false)
        .write(true)
        .read(true)
        .attributes(FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)
        .map_err(errno_from_ioerror)
}
