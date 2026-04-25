use libloading::{AsFilename, Library, Symbol};

// A struct for loading a library
pub struct PluginLoader {
    lib: Library,
}

impl PluginLoader {
    // Creates new PluginLoader instance
    // Takes a file path to a library
    pub fn new(path: impl AsFilename) -> Result<PluginLoader, libloading::Error> {
        let lib = unsafe { Library::new(path)? };
        Ok(Self { lib })
    }

    // Trying to load a function from a library
    // P - function signature must be specified as is
    // in a library with compatible data types
    pub fn load<'a, P: Send + Sync>(
        &'a self,
        name: &str,
    ) -> Result<Symbol<'a, P>, libloading::Error> {
        unsafe { self.lib.get(name.as_bytes()) }
    }
}
