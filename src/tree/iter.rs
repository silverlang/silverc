use std::fmt::Display;

use crate::tree::walk::TreeWalk;
use crate::tree::Tree;

impl<'tree, T> IntoIterator for &'tree Tree<T>
    where T: Clone + Display
{
    type Item = &'tree Tree<T>;

    type IntoIter = TreeWalk<'tree, T>;

    fn into_iter(self) -> Self::IntoIter {
        TreeWalk::new(self)
    }
}

impl<'tree, T> Iterator for TreeWalk<'tree, T>
where
    T: 'tree + Clone + Display,
{
    type Item = &'tree Tree<T>;

    fn next(&mut self) -> Option<&'tree Tree<T>> {
        let ret = self.stack.pop();
        return if let Some(next) = ret {
            self.current = next;
            match next {
                Tree::Leaf(_) => Some(next),
                Tree::Branch(_, ref children) => {
                    let mut children = children.iter().map(|child| child).collect();
                    self.stack.append(&mut children);
                    Some(next)
                }
            }
        } else {
            None
        };
    }
}

impl<'tree, T> FromIterator<&'tree Tree<T>> for TreeWalk<'tree, T>
where
    T: 'tree + Clone + Display,
{
    fn from_iter<I: IntoIterator<Item = &'tree Tree<T>>>(iter: I) -> Self {
        let mut root: Option<&'tree Tree<T>> = Option::None;
        for node in iter {
            root = Some(node);
            break;
        }

        TreeWalk::new(root.unwrap())
    }
}
