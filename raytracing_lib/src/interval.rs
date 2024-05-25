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

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, t: f64) -> bool {
        self.min <= t && t <= self.max
    }

    pub fn surrounds(&self, t: f64) -> bool {
        self.min <= t && t <= self.max
    }

    pub fn clamp(&self, t: f64) -> f64 {
        t.clamp(self.min, self.max)
    }

    pub fn expand(&self, t: f64) -> Self {
        let padding = t / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

// pub const EMPTY: Interval = Interval {
//     min: std::f64::INFINITY,
//     max: std::f64::NEG_INFINITY,
// };

// pub const UNIVERSE: Interval = Interval {
//     min: std::f64::NEG_INFINITY,
//     max: std::f64::INFINITY,
// };
