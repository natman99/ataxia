use crate::{
    feature::{Feature, HasFeature},
    sheet::source::{HasSource, Source},
};

pub struct Feat {
    pub name: String,
    pub desc: String,
    feature: Option<Feature>,
    source: Option<Source>,
}

impl HasSource for Feat {
    fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }

    fn add_source(&mut self, source: Source) {
        self.source = Some(source);
    }
}

impl HasFeature for Feat {
    fn apply(&self, sheet: &mut super::Character) {
        if let Some(s) = &self.feature {
            for i in &s.effects {
                i.apply(sheet);
            }
        }
    }

    fn add(&mut self, feature: super::feature::Effect) {
        if self.feature.is_none() {
            self.feature = Some(Feature {
                source: Some(self.source.clone().unwrap_or_else(|| {
                    Source::Single("Please add source before features".to_string())
                })),
                effects: vec![],
            })
        }

        self.feature.as_mut().unwrap().effects.push(feature);
    }
}
