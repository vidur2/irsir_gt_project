pub struct Coordinate {
    t: f32,
    s: f32,
    i: f32,
    r: f32
}

impl Coordinate {
    pub fn new(t: f32, s: f32, i: f32, r: f32) -> Self {
        return Self {
            t,
            s,
            i,
            r
        }
    }

    pub fn get_t(&self) -> f32 {
        return self.t
    }

    pub fn get_s(&self) -> f32 {
        return self.s
    }

    pub fn get_i(&self) -> f32 {
        return self.i
    }

    pub fn get_r(&self) -> f32 {
        return self.r
    }
}