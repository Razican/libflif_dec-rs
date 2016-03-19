use libc::{int32_t, int64_t, uint32_t, c_char, size_t, c_void};

pub enum FlifDecoder {}

extern "C" {
    pub fn flif_create_decoder() -> *mut FlifDecoder;
    pub fn flif_abort_decoder(decoder: *mut FlifDecoder);
    pub fn flif_destroy_decoder(decoder: *mut FlifDecoder);

    // pub fn flif_decoder_set_crc_check(decoder: *mut FlifDecoder, crc_check: int32_t);
    pub fn flif_decoder_set_quality(decoder: *mut FlifDecoder, quality: int32_t);
    pub fn flif_decoder_set_scale(decoder: *mut FlifDecoder, scale: uint32_t);
    pub fn flif_decoder_set_resize(decoder: *mut FlifDecoder, width: uint32_t, height: uint32_t);
    pub fn flif_decoder_set_callback(decoder: *mut FlifDecoder,
                                     callback: extern "C" fn(quality: int32_t, bytes_read: int64_t)
                                                             -> uint32_t);
    pub fn flif_decoder_set_first_callback_quality(decoder: *mut FlifDecoder, quality: int32_t);

    pub fn flif_decoder_decode_file(decoder: *mut FlifDecoder, filename: *const c_char) -> int32_t;
    pub fn flif_decoder_decode_memory(decoder: *mut FlifDecoder,
                                      buffer: *const c_void,
                                      buffer_size_bytes: size_t)
                                      -> int32_t;
    pub fn flif_decoder_num_images(decoder: *mut FlifDecoder) -> size_t;
    pub fn flif_decoder_num_loops(decoder: *mut FlifDecoder) -> int32_t;
// pub fn flif_decoder_get_image(decoder: *mut FlifDecoder, index: size_t) -> *const FLIF_IMAGE;
}
