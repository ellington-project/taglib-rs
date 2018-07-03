use taglib_sys::*;

use std::ffi::NulError;

use std::os::raw::c_void;

use std::str::Utf8Error;

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;


type StringReadError = Result<String, Utf8Error>;

type StringWriteError = Result<(), NulError>; 

#[derive(Debug)]
pub struct TagLibTag {
    tag: *mut TagLib_Tag
}

impl TagLibTag { 
    pub fn from_ptr(ptr: *mut TagLib_Tag) -> TagLibTag { 
        TagLibTag { tag: ptr }
    }

    fn read_and_parse(c_string_pointer: *mut c_char) -> StringReadError {
        unsafe {
        let str_slice = CStr::from_ptr(c_string_pointer);
            // try and parse that ptr into a string
            let str_res : StringReadError = str_slice.to_str().map(|s| s.to_owned());
            // free the pointer - TODO: Make this optional!
            taglib_free(c_string_pointer as *mut c_void);
            // and return the owned string
            str_res
        }
    }

    pub fn title(self: &Self) -> StringReadError {
        unsafe {
            Self::read_and_parse(taglib_tag_title(self.tag))
        }
    }

    pub fn artist(self: &Self) -> StringReadError {
        unsafe {
            Self::read_and_parse(taglib_tag_artist(self.tag))
        }
    }

    pub fn album(self: &Self) -> StringReadError {
        unsafe {
            Self::read_and_parse(taglib_tag_album(self.tag))
        }
    }

    pub fn comment(self: &Self) -> StringReadError {
        unsafe {
            Self::read_and_parse(taglib_tag_comment(self.tag))
        }
    }

    pub fn genre(self: &Self) -> StringReadError {
        unsafe {
            Self::read_and_parse(taglib_tag_genre(self.tag))
        }
    }

    pub fn year(self: &Self) -> Option<u32> {
        unsafe {
            match taglib_tag_year(self.tag) {
                0 => None,
                v => Some(v)
            }
        }
    }

    pub fn track(self: &Self) -> Option<u32> {
        unsafe {
            match taglib_tag_track(self.tag) {
                0 => None,
                v => Some(v)
            }
        }
    }

    pub fn bpm(self: &Self) -> Option<u32> {
        unsafe {
            match taglib_tag_bpm(self.tag) {
                0 => None,
                v => Some(v)
            }
        }
    }

    pub fn set_title(self: &Self, title: &String) -> StringWriteError { 
        unsafe {
            let title_ptr : *const c_char = match CString::new(title.as_str()) {
                Ok(cstr) => cstr.as_ptr(), 
                Err(e) => return Err(e)
            };
            taglib_tag_set_title(self.tag, title_ptr);
        }
        Ok(())
    }

    pub fn set_artist(self: &Self, artist: &String) -> StringWriteError { 
        unsafe {
            let artist_ptr : *const c_char = match CString::new(artist.as_str()) {
                Ok(cstr) => cstr.as_ptr(), 
                Err(e) => return Err(e)
            };
            taglib_tag_set_artist(self.tag, artist_ptr);
        }
        Ok(())
    }

    pub fn set_album(self: &Self, album: &String) -> StringWriteError { 
        unsafe {
            let album_ptr : *const c_char = match CString::new(album.as_str()) {
                Ok(cstr) => cstr.as_ptr(), 
                Err(e) => return Err(e)
            };
            taglib_tag_set_album(self.tag, album_ptr);
        }
        Ok(())
    }

    pub fn set_comment(self: &Self, comment: &String) -> StringWriteError { 
        unsafe {
            let comment_ptr : *const c_char = match CString::new(comment.as_str()) {
                Ok(cstr) => cstr.as_ptr(), 
                Err(e) => return Err(e)
            };
            taglib_tag_set_comment(self.tag, comment_ptr);
        }
        Ok(())
    }

    pub fn set_genre(self: &Self, genre: &String) -> StringWriteError { 
        unsafe {
            let genre_ptr : *const c_char = match CString::new(genre.as_str()) {
                Ok(cstr) => cstr.as_ptr(), 
                Err(e) => return Err(e)
            };
            taglib_tag_set_genre(self.tag, genre_ptr);
        }
        Ok(())
    }

    pub fn set_year(self: &Self, year: u32) -> () { 
        unsafe {
            taglib_tag_set_year(self.tag, year);
        }
    }

    pub fn set_track(self: &Self, track: u32) -> () { 
        unsafe {
            taglib_tag_set_track(self.tag, track);
        }
    }
}
