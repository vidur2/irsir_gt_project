use crate::coordinate::Coordinate;

pub trait Estimator<T> {
    fn new<'b>(model: &'b T) -> Self;
    fn estimate(&mut self, start: Coordinate, stop: f64, step: f64);
    fn get_estimations(&self) -> Vec<Coordinate>;
}