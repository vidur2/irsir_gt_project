use crate::{coordinate::Coordinate, error::Error};

pub trait Estimator<'a, T: Model> {
    fn new(model: &'a T) -> Self;
    fn estimate(&mut self, start: Coordinate, stop: f64, step: f64);
    fn get_estimations(self) -> Vec<Coordinate>;
}

pub trait Model {
    fn model(&self, start: Coordinate, stop: f64, step: f64) -> Result<Vec<Coordinate>, Error>;
}