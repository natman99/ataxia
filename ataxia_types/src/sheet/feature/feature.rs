use crate::{Character, sheet::source::HasSource};

pub trait HasFeature: HasSource {
    fn apply(&self, sheet: &mut Character);
    fn add(&mut self, feature: super::Effect);
}
