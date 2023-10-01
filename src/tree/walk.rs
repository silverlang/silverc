use std::fmt::{Debug, Display};

use crate::tree::Tree;

#[derive(Debug, Clone)]
pub struct TreeWalk<'tree, T>
where
    T: 'tree + Clone + Display,
{
    pub(crate) stack: Vec<&'tree Tree<T>>,
    pub(crate) current: &'tree Tree<T>
}

impl<'tree, T> TreeWalk<'tree, T>
where
    T: Clone + Display,
{
    pub fn new(tree: &'tree Tree<T>) -> Self {
        let stack = vec![tree];
        Self {
            stack,
            current: tree
        }
    }

    pub fn leaves(self) -> Vec<&'tree Tree<T>> {
        if let Tree::Branch(_, children) = self.current{
            children.iter().filter_map(|child|{
                if let Tree::Leaf(_) = child{
                    Some(child)
                }else{
                    None
                }
            })
            .collect()
        }else{
            vec![]
        }
    }

    pub fn branch<F: Fn(&T) -> bool>(self, f: F) -> Self {
        self.filter(|node| match node {
            Tree::Leaf(_) => false,
            Tree::Branch(ref value, _) => f(value),
        })
        .collect()
    }
}
