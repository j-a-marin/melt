/// Identifies a node in the transmission graph. Wraps the same string
/// identity used by `Observation::source_id` and the sensor `id` fields.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub String);

#[derive(Debug, Clone)]
pub struct Transmission {
    pub from: NodeId,
    pub to: NodeId,
    pub weight: f64,
}
