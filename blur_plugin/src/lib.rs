use std::{
    ffi::{CStr, c_char, c_uchar, c_uint},
    os::raw::c_int,
    slice::from_raw_parts_mut,
};

use image::imageops::GaussianBlurParameters;
use image::{DynamicImage, RgbaImage};
use serde_json;

#[unsafe(no_mangle)]
unsafe extern "C" fn process_image(
    width: c_uint,
    height: c_uint,
    rgb_data: *mut c_uchar,
    params: *const c_char,
) -> c_int {
    // check is a valid pointer
    if rgb_data.is_null() {
        eprintln!("rgb data is null pointer");
        return -1;
    }
    if params.is_null() {
        eprintln!("params is null pointer");
        return -1;
    }
    // We assume that we get null terminated C string
    let cparams = unsafe { CStr::from_ptr(params) };
    let json_str = match cparams.to_str() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("failed to cast cstr to str: {e}");
            return -1;
        }
    };
    let params_json: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("failed to parse params as json. {e}");
            return -1;
        }
    };
    let radius = match params_json.get("radius") {
        Some(v) => match v.as_f64() {
            Some(f) => f,
            None => {
                eprintln!("radius param is invalid. epxected float f64");
                return -1;
            }
        },
        None => {
            let default = 1.3;
            println!("No radius param. Using default {default}");
            default
        }
    };
    let iterations = match params_json.get("iterations") {
        Some(v) => match v.as_u64() {
            Some(n) => n,
            None => {
                eprintln!("iterations param is not valid. expected u64");
                return -1;
            }
        },
        None => {
            let default = 1;
            println!("No iterations param. Using default value {default}");
            default
        }
    };
    let buf_size = (width * height * 4) as usize;
    let image_slice = unsafe { from_raw_parts_mut(rgb_data, buf_size) };

    let mut image = match RgbaImage::from_raw(width, height, image_slice.to_vec()) {
        Some(rgbi) => DynamicImage::ImageRgba8(rgbi),
        None => {
            eprintln!("failed to create rgba image from raw vec");
            return -1;
        }
    }
    .blur_advanced(GaussianBlurParameters::new_from_radius(radius as f32));
    for _ in 0..iterations - 1 {
        image = match RgbaImage::from_raw(width, height, image.as_bytes().to_vec()) {
            Some(rgbai) => DynamicImage::ImageRgba8(rgbai)
                .blur_advanced(GaussianBlurParameters::new_from_radius(radius as f32)),
            None => {
                eprintln!("failed create rgba image from raw vec");
                return -1;
            }
        };
    }
    image_slice.copy_from_slice(image.as_bytes());
    0
}

#[cfg(test)]
mod tests {
    use image::ImageReader;

    use super::*;
    use std::ffi::CString;
    use std::str::FromStr;

    #[test]
    fn it_works() {
        let radius = 3.0;
        let iterations = 2;
        let json = serde_json::json!({
            "radius": radius,
            "iterations": iterations,
        });
        let params = CString::from_str(&json.to_string()).unwrap();
        let image_path = "../img/snow_leopard1.png";
        let mut image = image::ImageReader::with_format(
            ImageReader::open(image_path).unwrap().into_inner(),
            image::ImageFormat::Png,
        )
        .decode()
        .unwrap();
        unsafe {
            process_image(
                image.width(),
                image.height(),
                image.as_mut_rgba8().unwrap().as_mut_ptr(),
                params.as_ptr(),
            )
        };

        image
            .save_with_format(
                format!("newimage_r{}_i{}.png", radius, iterations),
                image::ImageFormat::Png,
            )
            .unwrap();
    }
}
