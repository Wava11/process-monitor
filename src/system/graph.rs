use crate::binding::StageIdentifier;

use super::stage::{Stage, StageDescription};

type Edge = (usize, Option<usize>);

pub struct Graph {
    stages_vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl<'a> From<Vec<Stage<'a>>> for Graph {
    fn from(value: Vec<Stage>) -> Self {
        Self {
            stages_vertices: value.iter().map(Vertex::from).collect(),
            edges: value
                .into_iter()
                .map(|stage: &Stage| -> Vec<Edge> {
                    stage
                        .next_stages
                        .into_iter()
                        .enumerate()
                        .map(|(index, next_stage)| {
                            let next_stage_vertex_index = value
                                .into_iter()
                                .position(|stage| stage.identifier == next_stage.identifier);
                            (index, next_stage_vertex_index)
                        })
                        .collect::<Vec<Edge>>()
                })
                .flatten()
                .collect(),
        }
    }
}

struct Vertex {
    description: Option<StageDescription>,
    identifer: StageIdentifier,
}

impl<'a> From<&Stage<'a>> for Vertex {
    fn from(value: &Stage) -> Self {
        Vertex {
            description: value.description,
            identifer: value.identifier,
        }
    }
}
