use crate::{coordinate::Coordinate, ir_sir::IrSir};

pub struct Euler<'a> {
    estimations: Vec<Coordinate>,
    irsir: &'a IrSir,
}

impl<'a> Euler<'a> {
    pub fn new(irsir: &'a IrSir) -> Self {
        return Self {
            irsir, 
            estimations: Vec::new(),
        }
    }

    pub fn estimate(&mut self, start: Coordinate, stop: f64, step: f64) {
        let t = start.get_t() + step;
        let coord = Coordinate::new(
            t,
            start.get_s() + self.irsir.ds_dt(start.get_s(), start.get_i()) * (t),
            start.get_i() + self.irsir.di_dt(start.get_s(), start.get_i(), start.get_r()) * (t),
            start.get_r() + self.irsir.dr_dt(start.get_i(), start.get_r()) * (t),
        );
        self.estimations.push(start);

        if stop > t {
            self.estimate(coord, stop, step);
        }
    }

    pub fn get_estimations(self) -> Vec<Coordinate> {
        return self.estimations;
    }
}