use std::collections::{HashMap, HashSet};

mod graph;
mod process;
mod stage;
use crate::binding::{Binding, StageIdentifier};
use process::Process;
use stage::Stage;

pub struct System {
    bindings: Vec<Binding>,
}

impl System {
    pub fn new(binding: Vec<Binding>) -> Self {
        System { bindings: binding }
    }

    pub fn generate_stages(&self) -> Vec<Stage> {
        let mut stages: Vec<Stage> = self.bindings.into_iter().map(|b| Stage::from(b)).collect();
        self.bindings.into_iter().map(|binding| {
            let stage = stages
                .iter_mut()
                .find(|stage| stage.identifier == binding.identifier)
                .expect("binding is not present in the stages list");

            stages
                .iter()
                .filter(|possible_next_stage| {
                    binding.targets.contains(&possible_next_stage.identifier)
                })
                .map(|next_stage| stage.add_next_stage(next_stage))
                .collect::<()>();
        });
        stages
    }
    // pub fn generate_processes(&self) -> Vec<Process> {
    //     let mut bindings_map = self.calculate_bindings_map();
    //     let mut stages_map = HashMap::<StageIdentifier, &Stage>::new();
    //     self.bindings.iter().map(|binding| {

    //         let stage_entry = stages_map
    //             .entry(binding.identifier)
    //             .or_insert(&Stage::from(binding));

    //         binding.targets.into_iter().map(|next_binding|{
    //             let next_stage = stages_map
    //             .entry(next_binding)
    //             .or_insert(bindings_map.get(next_binding).unwrap_or(Stage::));
    //             next_stage.
    //         })
    //     });
    // }

    fn calculate_bindings_map(&self) -> HashMap<StageIdentifier, Binding> {
        self.bindings
            .into_iter()
            .map(|b| (b.identifier, b))
            .collect()
    }
}
