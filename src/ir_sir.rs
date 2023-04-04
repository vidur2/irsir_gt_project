use crate::{euler::Euler, coordinate::Coordinate};
use crate::error::Error;

pub struct IrSir {
    alpha: f32,
    mu: f32,
    greek_letter_i_dont_know: f32,
    a: f32,
    beta: f32,
}

impl IrSir {
    pub fn new(alpha: f32, mu: f32, greek_letter_i_dont_know: f32, a: f32, beta: f32) -> Self {
        return Self {
            alpha,
            mu,
            greek_letter_i_dont_know,
            a,
            beta
        }
    }

    pub fn estimate(&self, start: Coordinate, stop: Coordinate, step: f32) -> Result<Vec<Coordinate>, Error> {
        if (stop.get_t() - start.get_t()) % step == 0. {
            return Err(Error::RangeError)
        }

        if step < 0. && stop.get_t() > start.get_t() {
            return Err(Error::NegativeSignError)
        }

        if step > 0. && stop.get_t() < start.get_t() {
            return Err(Error::PositiveSignError)
        }

        let mut euler = Euler::new(self, Box::new(Self::ds_dt), Box::new(Self::di_dt), Box::new(Self::dr_dt));
        euler.estimate(start, stop, step);

        return Ok(euler.get_estimations());
    }

    fn ds_dt(&self, s: f32, i: f32) -> f32{
        return self.a - self.alpha * s * i - self.mu * s;
    }

    fn di_dt(&self, s: f32, i: f32, r: f32) -> f32{
        return self.alpha * i * s - self.beta * i - self.greek_letter_i_dont_know * i * r - self.mu * i;
    }

    fn dr_dt(&self, i: f32, r: f32) -> f32{
        return self.beta * i + self.greek_letter_i_dont_know * i * r - self.mu * r;
    }
}