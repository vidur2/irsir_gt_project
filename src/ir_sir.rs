use crate::{euler::Euler, coordinate::Coordinate};
use crate::error::Error;

pub struct IrSir {
    alpha: f64,
    mu: f64,
    greek_letter_i_dont_know: f64,
    a: f64,
    beta: f64,
}

impl IrSir {
    pub fn new(alpha: f64, mu: f64, greek_letter_i_dont_know: f64, a: f64, beta: f64) -> Self {
        return Self {
            alpha,
            mu,
            greek_letter_i_dont_know,
            a,
            beta
        }
    }

    pub fn estimate(&self, start: Coordinate, stop: f64, step: f64) -> Result<Vec<Coordinate>, Error> {
        if (stop - start.get_t()) % step != 0. {
            return Err(Error::RangeError)
        }

        if step < 0. && stop > start.get_t() {
            return Err(Error::NegativeSignError)
        }

        if step > 0. && stop < start.get_t() {
            return Err(Error::PositiveSignError)
        }

        let mut euler = Euler::new(self);
        euler.estimate(start, stop, step);

        return Ok(euler.get_estimations());
    }

    pub fn ds_dt(&self, s: f64, i: f64) -> f64{
        return self.a - self.alpha * s * i - self.mu * s;
    }

    pub fn di_dt(&self, s: f64, i: f64, r: f64) -> f64{
        return self.alpha * i * s - self.beta * i - self.greek_letter_i_dont_know * i * r - self.mu * i;
    }

    pub fn dr_dt(&self, i: f64, r: f64) -> f64{
        return self.beta * i + self.greek_letter_i_dont_know * i * r - self.mu * r;
    }
}