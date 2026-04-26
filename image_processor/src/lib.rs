//! An example of a package of using
//! dynamic library like a plugin by
//! importing it by path

mod error;
mod plugin_loader;

use std::path::PathBuf;

use anyhow::anyhow;
pub use plugin_loader::PluginLoader;

pub fn make_plugin_path(plugin_path: PathBuf, plugin_name: &str) -> Result<PathBuf, anyhow::Error> {
    let mut path = PathBuf::from(&plugin_path).join(plugin_name);
    if cfg!(target_os = "windows") {
        path.set_extension("dll");
    } else if cfg!(target_os = "linux") {
        path.set_extension("so");
    } else {
        return Err(anyhow!("supported target_os are windows and linux"));
    }
    Ok(path)
}
