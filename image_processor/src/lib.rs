//! An example of a package of using
//! dynamic library like a plugin by
//! importing it by path

mod error;
mod plugin_loader;

pub use plugin_loader::PluginLoader;
