use std::path::Path;
use std::rc::Rc;

use super::{module_path::{self, ModulePath}, BytePos, deferred::DeferredSourceFile};

///This is simply a fat pointer to the contents of the source file, which is wrapped in an Rc to be
///referenced freely. This also contains the start of the file according to the absolute
///positioning system of the [super::SourceMap], which will be provided by the source map when a new
///[SourceFile] is created from a [super::deferred::DeferredSourceFile] object.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceCode{
    ///The start of the source file as an absolute position in the entire source map
    start: BytePos,
    ///The end of the source content as an absolute position in the entire source map
    end: BytePos,
    ///The source code itself wrapped in an Rc for distributing 
    ///
    ///The source code here cannot be borrowed when nothing else owns it
    pub content: Rc<str>
}

impl SourceCode{
    ///Constructs the [SourceCode] instance using a start pos and the data itself already wrapped
    ///in Rc
    pub fn new(start: BytePos, data: Rc<str>) -> Self{
        let end = data.len() + start;
        Self{
            start, end, content: data
        }
    }

    ///Attempts to get a slice of the internal data using start and end absolute positions, which
    ///will then be converted to relative positions in the source code
    pub fn get_at_pos(&self, pos_start: BytePos, pos_end: BytePos) -> std::result::Result<&str, String>{
        let file_range = self.start..self.end;
        if !file_range.contains(&pos_start){
            return Err(format!("{0} byte pos is not within the bounds of this file", pos_start));
        }
        if !file_range.contains(&pos_end){
            return Err(format!("{0} byte pos is not within the bounds of this file", pos_start));
        }
        let rel_start = pos_start - self.start;
        let rel_end = pos_end - self.start;
        Ok(&self.content.as_ref()[rel_start..rel_end])
    }
}

///A source file object which holds the finally loaded source code, its path, and the offset of the
///file within the entire [super::SourceMap].
#[derive(Clone, Debug)]
pub struct SourceFile{
    ///The path of the module itself
    pub module_path: module_path::ModulePath,
    ///The source code itself unless this is a directory
    pub source_code: Option<SourceCode>,
}

impl SourceFile{
    pub fn new(deferred: &DeferredSourceFile, offset: usize) -> Self{
        let path = deferred.path.clone();
        let path_str = path.to_string();
        let raw_path = Path::new(path_str.as_str());
        let canon_path = std::env::current_dir().unwrap().join(raw_path);
        if canon_path.is_dir(){
            Self{
                module_path: path,
                source_code: None,
            }
        }else{
            let source_code_data = deferred.source_code.load(canon_path);
            let source_code = SourceCode::new(offset, source_code_data.into());
            Self{
                module_path: path,
                source_code: Some(source_code)
            }
        }
    }

    pub fn get_offset(&self) -> Option<BytePos>{
        if let Some(source_code) = &self.source_code{
            Some(source_code.start)
        }else{
            None
        }
    }

    pub fn source_contains_pos(&self, pos: BytePos) -> bool{
        if let Some(source_code) = &self.source_code{
            (source_code.start..source_code.end).contains(&pos)
        }else{
            false
        }
    }

    pub fn with_source_code(&self, callback: impl Fn(&SourceCode)) -> &Self{
        if let Some(source_code) = &self.source_code{
            callback(source_code)
        }
        self
    }
}

impl std::fmt::Display for SourceFile{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
