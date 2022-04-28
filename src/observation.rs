use std::fmt::Formatter;
use core::fmt::Debug;
use core::fmt::Error;

pub struct Observation {
    pub position: Vec<f64>,
    pub cluster: Option<usize>,
    pub label: String,
}

impl Observation {
    pub fn new(position: Vec<f64>, label: String) -> Self {
        Observation {
            position,
            cluster: None,
            label,
        }
    }
}

impl Debug for Observation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Observation {{ label: {}, position: {:?}, cluster: {:?} }}", self.label, self.position, self.cluster.unwrap_or(0))
    }
}