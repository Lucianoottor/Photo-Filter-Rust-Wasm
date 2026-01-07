// Disable `std` library and `main` entrypoint, because they are not available in WebAssembly.
#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std, no_main)]

use parity_scale_codec::Decode;
use wasm_types::Picture;

extern crate alloc;
use alloc::vec;

#[global_allocator]
static mut ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

#[cfg(target_arch = "wasm32")]
#[panic_handler]
unsafe fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

pub mod ext {
    #[cfg(target_family = "wasm")]
    #[link(wasm_import_module = "env")]
    extern "C" {
        pub fn console_log(ptr: *const u8, len: u32);
        pub fn get_input(ptr: *mut u8, len: &mut u32);
    }

    #[cfg(not(target_family = "wasm"))]
    pub unsafe fn console_log(_ptr: *const u8, _len: u32) {}
    #[cfg(not(target_family = "wasm"))]
    pub unsafe fn get_input(_ptr: *mut u8, _len: &mut u32) {}
}

fn log(message: &str) {
    unsafe {
        ext::console_log(message.as_ptr(), message.len() as u32);
    }
}

const OK: u32 = 0;
const FAILURE: u32 = 1;

static mut METADATA_BUFFER: [u8; 65536] = [0u8; 65536];

#[no_mangle]
pub extern "C" fn alloc_pixel_buffer(width: u32, height: u32) -> *mut u8 {
    let size = (width * height * 4) as usize;
    let mut buffer = vec![0u8; size]; 
    let ptr = buffer.as_mut_ptr();
    
    core::mem::forget(buffer); 
    
    ptr 
}
fn apply_invert(pixels: &mut [u8]) {
    for i in (0..pixels.len()).step_by(4) {
        pixels[i]     = 255 - pixels[i];     // R
        pixels[i + 1] = 255 - pixels[i + 1]; // G
        pixels[i + 2] = 255 - pixels[i + 2]; // B
    }
}

fn apply_grayscale(pixels: &mut [u8]) {
    for i in (0..pixels.len()).step_by(4) {
        let gray = (pixels[i] as f32 * 0.2126 
                  + pixels[i+1] as f32 * 0.7152 
                  + pixels[i+2] as f32 * 0.0722) as u8;
        pixels[i] = gray;
        pixels[i+1] = gray;
        pixels[i+2] = gray;
    }
}

fn apply_sepia(pixels: &mut [u8]) {
    for i in (0..pixels.len()).step_by(4) {
        let r = pixels[i] as f32;
        let g = pixels[i+1] as f32;
        let b = pixels[i+2] as f32;

        pixels[i]   = (r * 0.393 + g * 0.769 + b * 0.189).min(255.0) as u8;
        pixels[i+1] = (r * 0.349 + g * 0.686 + b * 0.168).min(255.0) as u8;
        pixels[i+2] = (r * 0.272 + g * 0.534 + b * 0.131).min(255.0) as u8;
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref, static_mut_refs)]
pub extern "C" fn photo_filter(pointer: *mut u8, total_bytes: usize, filter_type: u32) {
    let pixels = unsafe { core::slice::from_raw_parts_mut(pointer, total_bytes) };
    match filter_type {
        0 => apply_grayscale(pixels),
        1 => apply_invert(pixels),
        2 => apply_sepia(pixels),
        _ => (), 
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref, static_mut_refs)]
pub unsafe extern "C" fn call(input_size: u32, filter_type: u32) -> u32 {
    let mut length = input_size;


    ext::get_input(METADATA_BUFFER.as_mut_ptr(), &mut length);


    let mut cursor = &METADATA_BUFFER[..length as usize];
    let picture = match Picture::decode(&mut cursor) {
        Ok(p) => p,
        Err(_) => {
            log("Erro: Metadata corrompido.");
            return FAILURE;
        }
    };

 
    let total_bytes = (picture.width * picture.height * 4) as usize;
    let pointer = picture.data_ptr as *mut u8;

    photo_filter(pointer, total_bytes, filter_type);
    OK
}