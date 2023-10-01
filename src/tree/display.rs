use std::fmt::Display;

use crate::tree::Tree;

struct TreeDisplay {
    level: u8,
}

impl TreeDisplay {
    fn display<T>(&mut self, tree: &Tree<T>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    where
        T: Clone + Display,
    {
        for _ in 0..self.level {
            let _ = write!(f, "    ");
        }
        match tree {
            Tree::Leaf(data) => {
                write!(f, "|-{}\n", data)
            }
            Tree::Branch(data, children) => {
                let _ = write!(f, "|-{}\n", data);
                self.level += 1;
                children.iter().for_each(|child| {
                    let _ = self.display(child, f);
                });
                self.level -= 1;
                Ok(())
            }
        }
    }
}
impl<T> std::fmt::Display for Tree<T>
where
    T: Clone + Display,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        TreeDisplay { level: 0 }.display(self, formatter)
    }
}
