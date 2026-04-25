use std::{
    ffi::c_uchar,
    os::raw::{c_char, c_int, c_uint},
    path::PathBuf,
    str::FromStr,
    time::Instant,
};

use anyhow::anyhow;
use clap::Parser;
use image::ImageReader;

use log::{error, info};

use image_processor::PluginLoader;

#[derive(Debug, Parser)]
struct ImageArgs {
    #[arg(long, help = "image path")]
    input: PathBuf,
    #[arg(long, help = "image output path")]
    output: PathBuf,
    #[arg(long, help = "plugin name")]
    plugin: String,
    #[arg(long, help = "path to a file with params")]
    params: PathBuf,
    #[arg(long, help = "plugin directory location")]
    plugin_path: PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let args = ImageArgs::parse();
    let image = match ImageReader::open(&args.input) {
        Ok(img) => img,
        Err(e) => {
            let img_path = args.input.to_str().unwrap_or("");
            return Err(anyhow!("failed to open image: {}. {e}", img_path));
        }
    };
    let plugin_path = {
        let mut path = PathBuf::from(&args.plugin_path).join(&args.plugin);
        path.set_extension("so");
        path
    };
    info!(
        "trying to open lib with path: {}",
        plugin_path.to_string_lossy()
    );
    let plugin_loader = PluginLoader::new(plugin_path)?;
    let func = plugin_loader.load::<unsafe extern "C" fn(
        width: c_uint,
        height: c_uint,
        rgb_data: *mut c_uchar,
        params: *const c_char,
    ) -> c_int>("process_image")?;
    let mut image = image.decode()?.into_rgba8();
    let (width, height) = image.dimensions();
    let params = std::fs::read_to_string(&args.params)
        .map_err(|e| anyhow!("failed to open params file. {e}"))?;
    let cstr = std::ffi::CString::from_str(&params)?;
    let timeit = Instant::now();
    let process_result = unsafe { func(width, height, image.as_mut_ptr(), cstr.as_ptr()) };
    info!("process is finished in {}ms", timeit.elapsed().as_millis());
    if process_result == 0 {
        image.save_with_format(args.output, image::ImageFormat::Png)?;
    } else {
        error!(
            "process is interrupted with error. process result code {}",
            process_result
        );
    }
    Ok(())
}
