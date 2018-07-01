// std library imports
use std::ffi::CString;
use std::ffi::NulError;
use std::path::PathBuf;

// taglib-sys imports
use taglib_sys::taglib_set_string_management_enabled;
use taglib_sys::taglib_file_free;
use taglib_sys::taglib_file_new;
use taglib_sys::taglib_file_save;
use taglib_sys::taglib_file_is_valid;
use taglib_sys::taglib_file_tag;
use taglib_sys::TagLib_File;

// intra library imports
use taglib_tag::TagLibTag;

#[derive(Debug)]
pub struct TagLibFile {
    file_handle: *mut TagLib_File,
    tag: TagLibTag,
}

pub enum FileError {
    OpenFailure,
    SaveFailure,
    PathAsString,
    NullPathString(NulError),
    InvalidTagFile
}

impl TagLibFile {
    pub fn new(filename: PathBuf) -> Result<TagLibFile, FileError> {
        // get the filename as a string, then a c string
        let str_filename = match filename.to_str() {
            Some(s) => s,
            None => return Err(FileError::PathAsString),
        };
        let cs_filename = match CString::new(str_filename) {
            Ok(cs) => cs,
            Err(ne) => return Err(FileError::NullPathString(ne)),
        };
        unsafe {
            // start off by setting the string management options 
            taglib_set_string_management_enabled(false as i32);
            // try to open the file using the ffi
            let file_ptr = taglib_file_new(cs_filename.as_ptr());
            // Todo: Should the struct member be a reference instead?
            if file_ptr.is_null() {
                return Err(FileError::OpenFailure);
            } else {
                // Check to see if the tag file is valid (true/false as int)
                if taglib_file_is_valid(file_ptr) == 0 { 
                    return Err(FileError::InvalidTagFile)
                }
                // pub fn taglib_file_is_valid(file: *const TagLib_File) -> ::std::os::raw::c_int;
                // Get the tag. We want to do this here, so that any references to it only live as long as the file (which is dropped through the drop trait)
                let tag_ptr = taglib_file_tag(file_ptr);
                return Ok(TagLibFile {
                    file_handle: file_ptr,
                    tag: TagLibTag::from_ptr(tag_ptr),
                });
            }
        }
    }

    pub fn save(self: &Self) -> Result<(), FileError> { 
        unsafe {
            let status_code = taglib_file_save(self.file_handle);
            // status code returns true on success, so compare with 0/non-zero
            if status_code == 0 { 
                Err(FileError::SaveFailure)
            } else { 
                Ok(())
            }
        }
    }

    // return a reference to the tag that only lives as long as the file
    pub fn tag(self: &Self) -> &TagLibTag { 
        &self.tag
    }
}

impl Drop for TagLibFile {
    fn drop(&mut self) {
        println!("Dropping!");
        // free the taglib file!
        unsafe {
            taglib_file_free(self.file_handle);
        }
    }
}
