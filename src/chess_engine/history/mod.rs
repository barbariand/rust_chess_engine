use std::ops::Deref;

use crate::chess_engine::board::actions::Actions;

#[derive(Default, Debug)]
pub struct History(pub Vec<Actions>);
impl History {
    pub fn add(&mut self, action: Actions) {
        self.0.push(action);
    }
}
impl<'a> IntoIterator for &'a History {
    type IntoIter = std::slice::Iter<'a, Actions>;
    type Item = &'a Actions;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
