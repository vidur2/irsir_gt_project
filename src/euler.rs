use crate::{coordinate::Coordinate, ir_sir::IrSir};

pub struct Euler<'a> {
    s: Box<dyn Fn(&IrSir, f32, f32) -> f32>,
    i: Box<dyn Fn(&IrSir, f32, f32, f32) -> f32>,
    r: Box<dyn Fn(&IrSir, f32, f32) -> f32>,
    estimations: Vec<Coordinate>,
    irsir: &'a IrSir,
}

impl<'a> Euler<'a> {
    pub fn new(irsir: &'a IrSir, s: Box<dyn Fn(&IrSir, f32, f32) -> f32>, i: Box<dyn Fn(&IrSir, f32, f32, f32) -> f32>, r: Box<dyn Fn(&IrSir, f32, f32) -> f32>) -> Self {
        return Self {
            irsir, 
            s,
            i,
            r,
            estimations: Vec::new(),
        }
    }

    pub fn estimate(&mut self, start: Coordinate, stop: Coordinate, step: f32) {
        let t = start.get_t() + step;
        let coord = Coordinate::new(
            t,
            start.get_s() + self.s.as_ref()(&self.irsir, start.get_s(), start.get_i()) * (t),
            start.get_i() + self.i.as_ref()(&self.irsir, start.get_s(), start.get_i(), start.get_r()) * (t),
            start.get_r() + self.r.as_ref()(&self.irsir, start.get_i(), start.get_r()) * (t),
        );
        self.estimations.push(start);

        if stop.get_t() > t {
            self.estimate(coord, stop, step);
        }
    }

    pub fn get_estimations(self) -> Vec<Coordinate> {
        return self.estimations;
    }
}