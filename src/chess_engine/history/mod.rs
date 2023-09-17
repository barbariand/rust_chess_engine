use super::pieces::Action;

#[derive(Default, Debug, Clone)]
pub struct History(pub Vec<Action>);
impl History {
    pub fn add(&mut self, action: Action) {
        self.0.push(action);
    }
}
impl<'a> IntoIterator for &'a History {
    type IntoIter = std::slice::Iter<'a, Action>;
    type Item = &'a Action;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
