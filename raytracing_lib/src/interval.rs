pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: std::f64::INFINITY,
            max: std::f64::NEG_INFINITY,
        }
    }
}

impl Interval {
    pub fn new(_min: f64, _max: f64) -> Self {
        Self {
            min: _min,
            max: _max,
        }
    }
}
