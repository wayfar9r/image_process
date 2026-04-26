use core::slice;
use std::{
    ffi::{CStr, c_char, c_uchar, c_uint},
    os::raw::c_int,
};

use image::{DynamicImage, RgbaImage};
use serde_json;

//
#[unsafe(no_mangle)]
unsafe extern "C" fn process_image(
    width: c_uint,
    height: c_uint,
    rgb_data: *mut c_uchar,
    params: *const c_char,
) -> c_int {
    // check is a valid pointer
    if rgb_data.is_null() {
        eprintln!("rgba data is null pointer");
        return -1;
    }
    if params.is_null() {
        eprintln!("params is null pointer");
        return -1;
    }
    let cparams = unsafe { CStr::from_ptr(params) };
    let json_str = match cparams.to_str() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("failed to convert cstr to str: {e}");
            return -1;
        }
    };
    let params_json: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("failed to parse params as json: {e}");
            return -1;
        }
    };
    let horizontal_flip = match params_json.get("horizontal") {
        Some(v) => match v.as_bool() {
            Some(b) => b,
            None => {
                eprintln!("failed to cast horizontal param to bool");
                return -1;
            }
        },
        None => false,
    };
    let vertical_flip = match params_json.get("vertical") {
        Some(v) => match v.as_bool() {
            Some(b) => b,
            None => {
                eprintln!("failed to cast vertical param to bool");
                return -1;
            }
        },
        None => false,
    };
    let buf_size = (width * height * 4) as usize;
    let image_slice = unsafe { slice::from_raw_parts_mut(rgb_data, buf_size) };

    let mut image = match RgbaImage::from_raw(width, height, image_slice.to_vec()) {
        Some(img) => DynamicImage::ImageRgba8(img),
        None => {
            eprintln!("failed create rgba image from raw vec");
            return -1;
        }
    };

    if horizontal_flip {
        image.apply_orientation(image::metadata::Orientation::FlipHorizontal);
    }
    if vertical_flip {
        image.apply_orientation(image::metadata::Orientation::FlipVertical);
    }
    image_slice.copy_from_slice(image.as_bytes());
    0
}

#[cfg(test)]
mod tests {
    use std::{ffi::CString, str::FromStr};

    use super::*;

    #[test]
    fn it_works() {
        let image_path = "../img/snow_leopard1.png";
        let mut image = image::open(image_path).unwrap();
        let json = CString::from_str(r#"{"horizontal": false, "vertical": true}"#).unwrap();
        unsafe {
            let image = image.as_mut_rgba8().unwrap();
            process_image(
                image.width(),
                image.height(),
                image.as_mut_ptr(),
                json.as_ptr(),
            )
        };
        image.save("result.png").unwrap();
    }
}
