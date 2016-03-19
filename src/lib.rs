extern crate libc;

use std::ffi::{OsStr, CString};

mod ffi;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    DecodingError,
}

pub struct FlifDecoder {
    decoder: *mut ffi::FlifDecoder,
}

impl Drop for FlifDecoder {
    fn drop(&mut self) {
        unsafe { ffi::flif_destroy_decoder(self.decoder) };
    }
}

impl FlifDecoder {
    pub fn new() -> FlifDecoder {
        FlifDecoder { decoder:  unsafe { ffi::flif_create_decoder() }}
    }

    pub fn abort(&mut self) {
        unsafe { ffi::flif_abort_decoder(self.decoder) };
    }

    pub fn set_quality(&mut self, quality: i32) {
        let q = if quality > 100 {
            100
        } else if quality < 0 {
            0
        } else {
            quality
        };
        unsafe { ffi::flif_decoder_set_quality(self.decoder, q) };
    }

    /// Set the scale of the decoder
    ///
    /// The scale must be `1` or power of two (1, 2, 4, 8, 16...)
    pub fn set_scale(&mut self, scale: u32) {
        let s = if scale % 2 == 0 || scale == 1 {
            scale
        } else if scale == 0 {
            panic!("Scale must be bigger than 0!")
        } else {
            panic!("Scale must be power of 2!")
        };
        unsafe { ffi::flif_decoder_set_scale(self.decoder, s) };
    }

    pub fn set_resize(&mut self, width: u32, height: u32) {
        unsafe { ffi::flif_decoder_set_resize(self.decoder, width, height) };
    }

    pub fn set_callback(&mut self, callback: extern "C" fn(quality: i32, bytes_read: i64) -> u32) {
        unsafe { ffi::flif_decoder_set_callback(self.decoder, callback) };
    }

    pub fn set_first_callback_quality(&mut self, quality: i32) {
        let q = if quality > 10_000 {
            10_000
        } else if quality < 0 {
            0
        } else {
            quality
        };
        unsafe { ffi::flif_decoder_set_first_callback_quality(self.decoder, q) };
    }

    pub fn decode_file<S: AsRef<OsStr>>(&self, path: S) -> Result<()> {
        let bytes = if cfg!(windows) {
            path.as_ref().to_str().unwrap().as_bytes()
        } else {
            use std::os::unix::ffi::OsStrExt;
            path.as_ref().as_bytes()
        };
        let c_path = CString::new(bytes).unwrap();
        match unsafe { ffi::flif_decoder_decode_file(self.decoder, c_path.as_ptr()) } {
            0 => Err(Error::DecodingError),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {}
}
