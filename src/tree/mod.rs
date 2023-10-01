pub mod display;
pub mod iter;
pub mod walk;

use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Tree<T>
where
    T: Clone + Display,
{
    Leaf(T),
    Branch(T, Vec<Tree<T>>),
}
