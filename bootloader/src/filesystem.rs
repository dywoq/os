use uefi::{
    CStr16, Error, Status, boot,
    proto::media::{
        file::{File, FileAttribute, FileHandle, FileMode},
        fs::SimpleFileSystem,
    },
};

/// Finds the file and returns its handle.
///
/// # Errors
///
/// See [uefi::boot::get_handle_for_protocol], [uefi::boot::open_protocol_exclusive],
/// [uefi::proto::media::file::Directory::open] for errors.
pub fn get_file_handle(
    path: &CStr16,
    file_mode: FileMode,
    file_attribute: FileAttribute,
) -> Result<FileHandle, Error> {
    let fs_handle = boot::get_handle_for_protocol::<SimpleFileSystem>()?;
    let mut fs = boot::open_protocol_exclusive::<SimpleFileSystem>(fs_handle)?;
    let mut dir = fs.open_volume()?;
    dir.open(path, file_mode, file_attribute)
}

/// Reads the file, turning it into regular file and copies the data to the buffer,
/// returns the amount of read bytes.
///
/// # Errors
///
/// See [get_file_handle], [uefi::proto::media::file::FileHandle::into_regular_file]
/// for errors.
pub fn read_file_into(path: &CStr16, buffer: &mut [u8]) -> Result<usize, Error> {
    let file = get_file_handle(path, FileMode::Read, FileAttribute::READ_ONLY)?;
    let mut regular_file = file.into_regular_file().ok_or(Status::UNSUPPORTED)?;
    regular_file.read(buffer)
}
