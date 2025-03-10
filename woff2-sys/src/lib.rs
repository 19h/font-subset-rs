#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate cpp;

use std::mem::MaybeUninit;

cpp! {{
    #include "woff2/encode.h"
    #include "woff2/decode.h"

    using std::string;
    using woff2::MaxWOFF2CompressedSize;
    using woff2::ConvertTTFToWOFF2;
    using woff2::WOFF2Params;
    using woff2::ComputeWOFF2FinalSize;
    using woff2::ConvertWOFF2ToTTF;
    
    // Define a wrapper for WOFF2StringOut since we can't directly use it from Rust
    bool ConvertWOFF2ToTTF_WithBuffer(
        const uint8_t* data, 
        size_t length, 
        uint8_t* result, 
        size_t* result_length
    ) {
        string output(*result_length, '\0');
        woff2::WOFF2StringOut out(&output);
        
        bool success = woff2::ConvertWOFF2ToTTF(data, length, &out);
        
        if (success) {
            *result_length = out.Size();
            // Copy the data from output to result
            if (*result_length > 0) {
                memcpy(result, output.data(), *result_length);
            }
        }
        
        return success;
    }
}}

#[link(name = "brotli")]
extern "C" {}

/// Converts TTF font data to WOFF2 format.
///
/// # Arguments
///
/// * `ttf_font_bytes` - The TTF font data to convert
/// * `additional_extended_metadata_bytes` - Additional metadata to include
/// * `brotli_quality` - Compression quality (0-11, recommended: 11)
/// * `allow_transforms` - Whether to allow font-specific transforms (recommended: true)
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - The converted WOFF2 font data
/// * `Err(())` - If conversion failed
pub fn convert_ttf_to_woff2(
    ttf_font_bytes: &[u8],
    additional_extended_metadata_bytes: &[u8],
    brotli_quality: u8,
    allow_transforms: bool,
) -> Result<Vec<u8>, ()> {
    debug_assert!(
        brotli_quality < 12,
        "brotli_quality should be between 0 and 11 inclusive"
    );
    
    let capacity = max_woff2_compressed_size(
        ttf_font_bytes.len(),
        additional_extended_metadata_bytes.len(),
    );
    let mut woff_font_bytes = Vec::with_capacity(capacity);
    let mut woff_font_bytes_length = MaybeUninit::uninit();
    
    let success = convert_ttf_to_woff2_internal(
        ttf_font_bytes.as_ptr(),
        ttf_font_bytes.len(),
        woff_font_bytes.as_mut_ptr(),
        woff_font_bytes_length.as_mut_ptr(),
        additional_extended_metadata_bytes.as_ptr(),
        additional_extended_metadata_bytes.len(),
        brotli_quality as i32,
        allow_transforms,
    );
    
    if success {
        unsafe {
            let length = woff_font_bytes_length.assume_init();
            woff_font_bytes.set_len(length);
        }
        woff_font_bytes.shrink_to_fit();
        Ok(woff_font_bytes)
    } else {
        Err(())
    }
}

#[inline(always)]
fn max_woff2_compressed_size(length: usize, extended_metadata_length: usize) -> usize {
    // This is a safer estimate that matches the behavior of MaxWOFF2CompressedSize
    // without requiring the actual TTF data
    length + 1024 + extended_metadata_length
}

fn convert_ttf_to_woff2_internal(
    data: *const u8,
    length: usize,
    result: *mut u8,
    result_length: *mut usize,
    extended_metadata: *const u8,
    extended_metadata_length: usize,
    brotli_quality: i32,
    allow_transforms: bool,
) -> bool {
    unsafe {
        cpp!([
            data as "const uint8_t *",
            length as "size_t",
            result as "uint8_t *",
            result_length as "size_t *",
            extended_metadata as "const char *",
            extended_metadata_length as "size_t",
            brotli_quality as "int",
            allow_transforms as "bool"
        ] -> bool as "bool" {
            string copyOfExtendedMetadata(extended_metadata, extended_metadata_length);
        
            struct WOFF2Params params;
            params.extended_metadata = copyOfExtendedMetadata;
            params.brotli_quality = brotli_quality;
            params.allow_transforms = allow_transforms;
        
            return ConvertTTFToWOFF2(data, length, result, result_length, params);
        })
    }
}

// Constants
const DEFAULT_MAX_SIZE: usize = 30 * 1024 * 1024;  // 30MB max size like in C++ code

/// Converts WOFF2 font data to TTF format.
///
/// # Arguments
///
/// * `woff2_font_bytes` - The WOFF2 font data to convert
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - The converted TTF font data
/// * `Err(())` - If conversion failed
pub fn convert_woff2_to_ttf(woff2_font_bytes: &[u8]) -> Result<Vec<u8>, ()> {
    // First, compute the maximum size of the output TTF data
    let max_size = compute_woff2_final_size(
        woff2_font_bytes.as_ptr(),
        woff2_font_bytes.len(),
    );
    
    if max_size == 0 || max_size > DEFAULT_MAX_SIZE {
        return Err(());
    }
    
    // Allocate a buffer for the result
    let mut ttf_font_bytes = Vec::with_capacity(max_size);
    let mut ttf_font_bytes_length = MaybeUninit::uninit();
    
    // Convert the WOFF2 data to TTF
    let success = convert_woff2_to_ttf_internal(
        woff2_font_bytes.as_ptr(),
        woff2_font_bytes.len(),
        ttf_font_bytes.as_mut_ptr(),
        ttf_font_bytes_length.as_mut_ptr(),
    );
    
    if success {
        unsafe {
            let length = ttf_font_bytes_length.assume_init();
            ttf_font_bytes.set_len(length);
        }
        ttf_font_bytes.shrink_to_fit();
        Ok(ttf_font_bytes)
    } else {
        Err(())
    }
}

fn compute_woff2_final_size(data: *const u8, length: usize) -> usize {
    unsafe {
        cpp!([
            data as "const uint8_t *",
            length as "size_t"
        ] -> usize as "size_t" {
            return std::min(ComputeWOFF2FinalSize(data, length), 
                           static_cast<size_t>(30 * 1024 * 1024));
        })
    }
}

fn convert_woff2_to_ttf_internal(
    data: *const u8,
    length: usize,
    result: *mut u8,
    result_length: *mut usize,
) -> bool {
    unsafe {
        cpp!([
            data as "const uint8_t *",
            length as "size_t",
            result as "uint8_t *",
            result_length as "size_t *"
        ] -> bool as "bool" {
            // Create a temporary buffer for the output
            std::string output;
            output.resize(30 * 1024 * 1024, 0);  // Use DEFAULT_MAX_SIZE
            
            // Create the WOFF2StringOut and perform conversion
            woff2::WOFF2StringOut out(&output);
            bool success = woff2::ConvertWOFF2ToTTF(data, length, &out);
            
            if (success) {
                // Get the actual size and copy data to result
                *result_length = out.Size();
                if (*result_length > 0) {
                    memcpy(result, output.data(), *result_length);
                }
            }
            
            return success;
        })
    }
}