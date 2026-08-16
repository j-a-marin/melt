#[derive(Debug, Clone)]
pub struct Falsifier {
    pub description: String,
    /// Deadline expressed as a logical tick on the same clock as
    /// `Observation::timestamp`, so the two can be compared directly.
    pub expected_by: u64,
}
