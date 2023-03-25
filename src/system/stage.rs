use std::collections::HashSet;

use crate::binding::{Binding, StageIdentifier};

#[derive(Debug, Eq, PartialEq)]
pub struct Stage<'a> {
    description: Option<StageDescription>,
    identifier: StageIdentifier,
    pub next_stages: HashSet<&'a Stage<'a>>,
}

impl<'a> std::hash::Hash for Stage<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.identifier.hash(state)
    }
}

impl<'a> From<&Binding> for Stage<'a> {
    fn from(value: &Binding) -> Self {
        Self {
            description: Some(StageDescription {
                executor: value.executor,
                name: value.stage_name,
            }),
            identifier: value.identifier,
            next_stages: HashSet::new(),
        }
    }
}

impl<'a> Stage<'a> {
    pub fn add_next_stage(&mut self, stage: &'a Stage) {
        self.next_stages.insert(stage);
    }
}

#[derive(Debug, Eq, PartialEq)]
struct StageDescription {
    name: String,
    executor: String,
}
