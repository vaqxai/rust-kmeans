pub trait EuclidDist {
    fn dist(&self, other: &Self) -> Option<f64>;
}

impl EuclidDist for Vec<f64> {
    fn dist(&self, other: &Self) -> Option<f64> {
        if self.len() != other.len() {
            return None;
        }

        let mut sum = 0.0;
        for i in 0..self.len() {
            sum += (self[i] - other[i]).powi(2);
        }

        Some(sum.sqrt())

    }
}