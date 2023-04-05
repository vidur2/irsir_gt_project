use std::fmt::Display;

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


impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(t: {}, s: {}, i: {}, r: {})", self.get_t(), self.get_s(), self.get_i(), self.get_r())?;
        return Ok(());
    }
}