pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn empty() -> Self {
        Interval {
            min: std::f64::INFINITY,
            max: -std::f64::INFINITY,
        }
    }

    pub fn universe() -> Self {
        Interval {
            min: -std::f64::INFINITY,
            max: std::f64::INFINITY,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, value: f64) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn surrounds(&self, value: f64) -> bool {
        self.min < value && value < self.max
    }

    pub fn clamp(&self, value: f64) -> f64 {
        if value < self.min {
            return self.min;
        }
        if value > self.max {
            return self.max;
        }
        return value;
    }
}
