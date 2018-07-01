use taglib_file::TagLibFile;
use taglib_sys::TagLib_Tag;

#[derive(Debug)]
pub struct TagLibTag {
    tag: *mut TagLib_Tag
}

impl TagLibTag { 
    pub fn from_ptr(ptr: *mut TagLib_Tag) -> TagLibTag { 
        TagLibTag { tag: ptr }
    }
    /// Returns a string with this tag's title.
    ///
    /// \note By default this string should be UTF8 encoded and its memory should be
    /// freed using taglib_tag_free_strings().
    // pub fn taglib_tag_title(tag: *const TagLib_Tag) -> *mut ::std::os::raw::c_char;
    pub fn title(self: &Self) -> () {
        
    }
    /// Returns a string with this tag's artist.
    ///
    /// \note By default this string should be UTF8 encoded and its memory should be
    /// freed using taglib_tag_free_strings().
    // pub fn taglib_tag_artist(tag: *const TagLib_Tag) -> *mut ::std::os::raw::c_char;
    pub fn artist(self: &Self) -> () {
        
    }
    /// Returns a string with this tag's album name.
    ///
    /// \note By default this string should be UTF8 encoded and its memory should be
    /// freed using taglib_tag_free_strings().
    // pub fn taglib_tag_album(tag: *const TagLib_Tag) -> *mut ::std::os::raw::c_char;
    pub fn album(self: &Self) -> () {
        
    }
    /// Returns a string with this tag's comment.
    ///
    /// \note By default this string should be UTF8 encoded and its memory should be
    /// freed using taglib_tag_free_strings().
    // pub fn taglib_tag_comment(tag: *const TagLib_Tag) -> *mut ::std::os::raw::c_char;
    pub fn comment(self: &Self) -> () {
        
    }
    /// Returns a string with this tag's genre.
    ///
    /// \note By default this string should be UTF8 encoded and its memory should be
    /// freed using taglib_tag_free_strings().
    // pub fn taglib_tag_genre(tag: *const TagLib_Tag) -> *mut ::std::os::raw::c_char;
    pub fn genre(self: &Self) -> () {
        
    }
    /// Returns the tag's year or 0 if year is not set.
    // pub fn taglib_tag_year(tag: *const TagLib_Tag) -> ::std::os::raw::c_uint;
    pub fn year(self: &Self) -> () {
        
    }
    /// Returns the tag's track number or 0 if track number is not set.
    // pub fn taglib_tag_track(tag: *const TagLib_Tag) -> ::std::os::raw::c_uint;
    pub fn track(self: &Self) -> () {
        
    }
    /// Returns the beats-per-minute (bpm) of the track; if there is no bpm
    /// set, or bpm tag in the metadata, this will return 0.
    // pub fn taglib_tag_bpm(tag: *const TagLib_Tag) -> ::std::os::raw::c_uint;
    pub fn bpm(self: &Self) -> () {
        
    }
    /// Sets the tag's title.
    ///
    /// \note By default this string should be UTF8 encoded.
    // pub fn taglib_tag_set_title(tag: *mut TagLib_Tag, title: *const ::std::os::raw::c_char);
    /// Sets the tag's artist.
    ///
    /// \note By default this string should be UTF8 encoded.
    // pub fn taglib_tag_set_artist(tag: *mut TagLib_Tag, artist: *const ::std::os::raw::c_char);
    /// Sets the tag's album.
    ///
    /// \note By default this string should be UTF8 encoded.
    // pub fn taglib_tag_set_album(tag: *mut TagLib_Tag, album: *const ::std::os::raw::c_char);
    /// Sets the tag's comment.
    ///
    /// \note By default this string should be UTF8 encoded.
    // pub fn taglib_tag_set_comment(tag: *mut TagLib_Tag, comment: *const ::std::os::raw::c_char);
    /// Sets the tag's genre.
    ///
    /// \note By default this string should be UTF8 encoded.
    // pub fn taglib_tag_set_genre(tag: *mut TagLib_Tag, genre: *const ::std::os::raw::c_char);
    /// Sets the tag's year.  0 indicates that this field should be cleared.
    // pub fn taglib_tag_set_year(tag: *mut TagLib_Tag, year: ::std::os::raw::c_uint);
    /// Sets the tag's track number.  0 indicates that this field should be cleared.
    // pub fn taglib_tag_set_track(tag: *mut TagLib_Tag, track: ::std::os::raw::c_uint);
}
