use std::path::Path;
use std::sync::Arc;
use std::rc::Rc;

///Represents a part of a whole project path, which is useful for recursing through the project and
///matching module symbols such as with processing and validating from-import statements
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModulePath{
    ///The parent module if it exists at all or not, wrapped in an Rc instance for referencing
    ///throughout the compiler
    parent: Option<Rc<ModulePath>>,
    ///The name of the module itself, used for easily checking against a module symbol in a
    ///from-import statement. This is wrapped in Arc so that multiple threads can access it
    name: Arc<str>,
}

impl ModulePath{
    ///Constructs a new instance of [ModulePath] using a `AsRef<Path>` reference and an optional parent
    pub fn new(path: impl AsRef<Path>, parent: Option<ModulePath>) -> Self{
        let path = path.as_ref();
        let name: Arc<str> = path
            .file_name()
            .map(|name| name.to_str()
                 .map(|str| str.into()))
            .flatten()
            .unwrap();
        let parent = parent.map(|parent| parent.into());
        Self{
            parent, name
        }
    }

    pub fn get_name(&self) -> Arc<str>{
        self.name.clone()
    }

    ///Creates a string from the module's name appended to the parent's (if it exists)
    pub fn to_string(&self) -> String{
        let mut parent = match self.parent{
            Some(ref parent) => {
                let mut parent = parent.to_string();
                parent.push_str("/");
                parent
            },
            None => "".into(),
        };
        parent.push_str(self.name.as_ref());
        parent
    }
}

impl std::fmt::Display for ModulePath{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mod_str = self.to_string();
        write!(f, "{0}", mod_str)
    }
}
