use std::{sync::Arc, collections::HashMap, path::Path, hash::{Hash, Hasher}};

use crate::tree::{Tree, walk::TreeWalk};

use self::{deferred::DeferredSourceFile, source_file::SourceFile};

pub mod deferred;
pub mod module_path;
pub mod source_file;

pub type BytePos = usize;

///This represents an the id of a source file, which currently is very simple but when we get
///incremental compilation, we can update this to rely on compilation sessions
///
///This is just a hash64 of the source file's project relative path
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SourceFileID(u64);

///A map of the project's entire source code, which allows for both tree walking and fetching the
///files themselves based on a distributed source file ID
///
///                SourceMap 
///              /         ^
///          Arc<Tree>  HashMap<SourceFileID, SourceFile>
///            / walk          ^ put
///     DeferredSourceFile  SourceFile
///            | load          ^ new
///     DeferredSourceCode  SourceCode
///           | load           ^ new
///       SourceCode ---------/
///                     new
///           
#[derive(Clone, Debug)]
pub struct SourceMap{
    ///An [Arc] wrapped [Tree] of [source_file::SourceFile]'s, which allows for brute force searching the source
    ///tree. This is the slower approach and is only used initially for referencing a source file.
    ///
    ///This is generally a more clunky way to hold the project structure, however, this can be used
    ///by the future plugin system to allow plugins to override the default project structure
    ///
    ///This is good for doing analysis on the project structure to ensure that the structure is
    ///correct
    pub source_tree: Arc<Tree<deferred::DeferredSourceFile>>,
    ///A map of ids to source files. This is used for fetching a source file that has already been
    ///referenced.
    source_id_map: HashMap<SourceFileID, source_file::SourceFile>
}

impl SourceMap{
    fn load_tree(root_path: impl AsRef<Path>, parent: Option<module_path::ModulePath>) -> std::result::Result<Tree<deferred::DeferredSourceFile>, &'static str>{
        let root_path = root_path.as_ref();
        if !root_path.exists(){
            return Err("Root path does not exist!");
        }
        if !root_path.is_dir(){
            return Err("Root path should be a directory");
        }

        let modpath = module_path::ModulePath::new(root_path, parent);
        let mut children = vec![];
        for entry in std::fs::read_dir(root_path).unwrap(){
            let entry = entry.unwrap().path();
            let child_node = if entry.is_dir(){
                Self::load_tree(entry, Some(modpath.clone()))?
            }else{
                let modpath = module_path::ModulePath::new(entry, Some(modpath.clone()));
                let deferred_sf = DeferredSourceFile::new(modpath);
                Tree::Leaf(deferred_sf)
            };
            children.push(child_node);
        }
        Ok(Tree::Branch(DeferredSourceFile::new(modpath), children))
    }

    pub fn new(root_path: impl AsRef<Path>) -> std::result::Result<Self, &'static str>{
        let tree = Self::load_tree(root_path, None)?;
        Ok(Self{
            source_tree: tree.into(),
            source_id_map: HashMap::new()
        })
    }

    fn get_offset(&self) -> usize{
        if self.source_id_map.is_empty(){
            0
        }else{
            self.source_id_map.iter()
                .last()
                .map(|(_, file)| file.get_offset())
                .flatten()
                .unwrap()
        }
    }

    pub fn get_file_with_pos(&mut self, pos: BytePos) -> Option<&source_file::SourceFile>{
        self.source_id_map.iter().find(|(_, module)| module.source_contains_pos(pos)).map(|(_, module)| module)
    }

    pub fn get_module(&mut self, module_path: module_path::ModulePath) -> Option<source_file::SourceFile>{
        if let Some((_, file)) = self.source_id_map.iter().find(|(_, file)| file.module_path == module_path){
            return Some(file.clone())
        }

        let mut walk: TreeWalk<DeferredSourceFile> = self.source_tree.into_iter();
        let file = walk
            .find(|tree|{
                match tree{
                    Tree::Leaf(file) => file.path == module_path,
                    Tree::Branch(dir, _) => dir.path == module_path
                }
            })
            .map(|tree| match tree{
                Tree::Leaf(file) => file,
                Tree::Branch(dir, _) => dir
            })
            .map(|module| SourceFile::new(module, self.get_offset()));
        if let Some(ref sf) = file{
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            let name = sf.module_path.get_name();
            name.hash(&mut hasher);
            let hash = hasher.finish();
            self.source_id_map.insert(SourceFileID(hash), sf.clone());
        }
        file
    }
}
