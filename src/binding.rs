#[derive(Eq, PartialEq, Hash)]
pub struct Binding {
    pub process_name: String,
    pub stage_name: String,
    pub identifier: StageIdentifier,
    pub queue: String,
    pub executor: String,
    pub targets: Vec<StageIdentifier>,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct StageIdentifier {
    pub exchange: String,
    pub topic: String,
}
