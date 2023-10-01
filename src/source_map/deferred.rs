use std::{rc::Rc, path::Path};

use crate::source_map::module_path;

#[derive(Clone, Debug)]
///A deferred source file, which simply represents a source file to be loaded later when needed
/// This is to ensure that if any source files in the project are not actually used, they don't get
/// loaded into memory which would be an expensive operation if there's refactoring going on.
pub struct DeferredSourceFile{
    ///The module path, which allows for recursive checking if needed
    pub path: module_path::ModulePath,
    ///The deferred source code, which is simply an empty struct that allows for calling methods on
    ///it to load the source code later
    pub source_code: DeferredSourceCode
}

impl DeferredSourceFile{
    ///The constructor for this struct
    pub fn new(path: module_path::ModulePath) -> Self{
        Self{
            path, source_code: DeferredSourceCode{}
        }
    }
}

impl std::fmt::Display for DeferredSourceFile{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeferredSourceFile{{ path = \"{0}\" }}", self.path)
    }
}

///An empty struct which simply acts as a means for interfacing with and representing source code
///which has yet to be loaded
///
///This is needed because we don't want to load source code right away in a project that may have
///files that are not yet or no longer being used. This is also good for ensuring that we only load
///the source code that we need at the moment
#[derive(Clone, Debug)]
pub struct DeferredSourceCode;

impl DeferredSourceCode{
    ///Checks whether a given path exists
    pub fn exists(&self, path: impl AsRef<Path>) -> bool{
        path.as_ref().exists()
    }

    ///Checks whether a given path is a file
    pub fn is_file(&self, path: impl AsRef<Path>) -> bool{
        path.as_ref().is_file()
    }

    ///Attempts to load the source code into an `Rc<str>` and return it to be passed to a
    ///[super::source_file::SourceFile] object
    pub fn load(&self, path: impl AsRef<Path>) -> Rc<str>{
        let data = std::fs::read_to_string(path).expect("Unable to read contents of file");
        data.into()
    }
}
