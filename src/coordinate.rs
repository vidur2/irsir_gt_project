#[derive(Debug)]
pub struct Coordinate {
    t: f64,
    s: f64,
    i: f64,
    r: f64
}

impl Coordinate {
    pub fn new(t: f64, s: f64, i: f64, r: f64) -> Self {
        return Self {
            t,
            s,
            i,
            r
        }
    }

    pub fn get_t(&self) -> f64 {
        return self.t
    }

    pub fn get_s(&self) -> f64 {
        return self.s
    }

    pub fn get_i(&self) -> f64 {
        return self.i
    }

    pub fn get_r(&self) -> f64 {
        return self.r
    }
}