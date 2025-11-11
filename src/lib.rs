use std::{ffi, ptr, sync};

mod bindings;
mod kernel;

static INIT_DECODER: sync::Once = sync::Once::new();
static INIT_ENCODER: sync::Once = sync::Once::new();

/// Returns the used rapidyenc version, e.g., `1.1.1`.
pub fn version() -> String {
    let version = unsafe { bindings::rapidyenc_version() };

    let [patch, minor, major, ..] = version.to_le_bytes();
    format!("{major}.{minor}.{patch}")
}

/// Returns the maximum **possible** length of yEnc encoded output, given an input of `length` bytes.
pub fn encode_max_length(length: usize, line_size: u32) -> usize {
    let line_size = i32::try_from(line_size).expect("line size overflows i32");
    unsafe { bindings::rapidyenc_encode_max_length(length, line_size) }
}

#[inline]
fn encode_init() {
    INIT_ENCODER.call_once(|| unsafe { bindings::rapidyenc_encode_init() });
}

#[inline]
fn decode_init() {
    INIT_DECODER.call_once(|| unsafe { bindings::rapidyenc_decode_init() });
}

/// Returns the kernel/ISA level used for encoding.
///
/// If the kernel is not recognized, the raw `i32` is returned.
pub fn encode_kernel() -> Result<kernel::Kernel, i32> {
    encode_init();

    let kernel_id = unsafe { bindings::rapidyenc_encode_kernel() };
    kernel::Kernel::try_from(kernel_id).map_err(|_| kernel_id)
}

/// Returns the kernel/ISA level used for decoding.
///
/// If the kernel is not recognized, the raw `i32` is returned.
pub fn decode_kernel() -> Result<kernel::Kernel, i32> {
    decode_init();

    let kernel_id = unsafe { bindings::rapidyenc_decode_kernel() as i32 };
    kernel::Kernel::try_from(kernel_id).map_err(|_| kernel_id)
}

/// Decodes the whole `buffer` in-place and truncates it to the decoded length.
pub fn decode(buffer: &mut Vec<u8>) {
    decode_init();

    let src = buffer.as_ptr() as *const ffi::c_void;
    let dest = buffer.as_mut_ptr() as *mut ffi::c_void;
    let src_length = buffer.len();

    // SATEFTY: The `src` and `dest` pointers are valid for `src_length` bytes.
    // The `src` and `dest` pointers are allowed to overlap.
    let decoded_len = unsafe { bindings::rapidyenc_decode(src, dest, src_length) };

    buffer.truncate(decoded_len);
}

/// Encodes `data` into a new allocation with the specified `line_size`.
pub fn encode(data: &[u8], line_size: u32) -> Vec<u8> {
    encode_init();

    let line_size = i32::try_from(line_size).expect("line size overflows i32");

    let max_length = encode_max_length(data.len(), line_size as u32);
    let mut encoded = vec![0u8; max_length];

    let src = data.as_ptr() as *const ffi::c_void;
    let dest = encoded.as_mut_ptr() as *mut ffi::c_void;
    let src_length = data.len();

    // SATEFTY: The `src` pointer is valid for `src_length` bytes.
    // The `dest` pointer is valid for `max_length` bytes.
    // The `src` and `dest` pointers do not overlap.
    let encoded_length = unsafe {
        bindings::rapidyenc_encode_ex(line_size, ptr::null_mut(), src, dest, src_length, 1)
    };

    encoded.truncate(encoded_length);
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &[u8] = include_bytes!("../test-data/cat.jpg");
    const DATA_YENC: &[u8] = include_bytes!("../test-data/cat.jpg.yenc");

    #[test]
    fn test_version() {
        // https://gitea.onlyusenet.com/bebop/rapidyenc/src/commit/47f67f5ae31455a4e7bb2566fb2b6c3c1b0105e9/rapidyenc.cc#L11-L13
        assert_eq!(version(), "1.1.1"); // version 1.1.1
    }

    #[test]
    fn test_encode_max_length() {
        // https://gitea.onlyusenet.com/bebop/rapidyenc/src/commit/47f67f5ae31455a4e7bb2566fb2b6c3c1b0105e9/rapidyenc.cc#L41-L54
        let max_length = encode_max_length(1000, 128);
        assert_eq!(max_length, 2066 + 2 * (1000 >> 6));
    }

    #[test]
    fn test_encode_kernel() {
        let kernel = encode_kernel();
        assert!(kernel.is_ok());
    }

    #[test]
    fn test_decode_kernel() {
        let kernel = decode_kernel();
        assert!(kernel.is_ok());
    }

    #[test]
    fn test_round_trip() {
        let mut buffer = encode(DATA, 128);
        decode(&mut buffer);
        assert_eq!(buffer.len(), DATA.len());
        assert_eq!(buffer, DATA);
    }

    #[test]
    fn test_encode_decoded() {
        let encoded = encode(DATA, 128);
        assert_eq!(encoded.len(), DATA_YENC.len());
        assert_eq!(encoded.as_slice(), DATA_YENC);
    }

    #[test]
    fn test_decode_encoded() {
        let mut data = DATA_YENC.to_vec();
        decode(&mut data);
        assert_eq!(data.len(), DATA.len());
        assert_eq!(data.as_slice(), DATA);
    }
}
