use std::collections::HashSet;

use crate::binding::StageIdentifier;

mod binding;
mod system;

fn main() {
    let mut set = HashSet::<StageIdentifier>::new();

    set.insert(StageIdentifier {
        exchange: String::from("e1"),
        topic: String::from("t1"),
    });
    set.insert(StageIdentifier {
        exchange: String::from("e2"),
        topic: String::from("t2"),
    });

    let mut other_set1 = HashSet::<&StageIdentifier>::new();
    let mut other_set2 = HashSet::<&StageIdentifier>::new();

    let stages: Vec<&StageIdentifier> = set.iter().collect();
    other_set1.insert(stages[0]);
    other_set2.insert(stages[0]);

    println!("{:?}", other_set1.iter());

    println!("Hello, world!");
}
