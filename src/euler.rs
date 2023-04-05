use crate::{coordinate::Coordinate, ir_sir::IrSir, estimator::{Estimator, Model}};

pub struct Euler<'a, T: Model> {
    estimations: Vec<Coordinate>,
    model: &'a T,
}

impl<'a> Estimator<'a, IrSir> for Euler<'a, IrSir> {
    fn new(model: &'a IrSir) -> Self {
        return Self {
            model, 
            estimations: Vec::new(),
        }
    }

    fn estimate(&mut self, start: Coordinate, stop: f64, step: f64) {
        let t = start.get_t() + step;
        let coord = Coordinate::new(
            t,
            start.get_s() + self.model.ds_dt(start.get_s(), start.get_i()) * (step),
            start.get_i() + self.model.di_dt(start.get_s(), start.get_i(), start.get_r()) * (step),
            start.get_r() + self.model.dr_dt(start.get_i(), start.get_r()) * (step),
        );
        self.estimations.push(start);

        if stop + step >= t {
            self.estimate(coord, stop, step);
        }
    }

    fn get_estimations(self) -> Vec<Coordinate> {
        return self.estimations;
    }
}