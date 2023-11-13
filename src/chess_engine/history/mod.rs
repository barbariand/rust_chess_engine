use std::ops::Deref;

use crate::chess_engine::board::actions::Action;

#[derive(Default,Debug)]
pub struct History(pub Vec<Box<dyn Action>>);
impl History {
    pub fn add(&mut self,action:impl Action){
        self.0.push(Box::new(action));
    }
    //pub fn from_algebraic
}
impl<'a> IntoIterator for &'a History{
    type IntoIter = std::slice::Iter<'a,Box<dyn Action>>;
    type Item = &'a Box<dyn Action>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
