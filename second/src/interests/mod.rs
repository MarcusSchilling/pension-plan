use rand_distr::{Distribution, Normal};
use std::iter::Iterator;

pub struct MeanReversion {
    mean_return: f64,
    volatility: f64,
    current_return: f64,
}

impl MeanReversion {
    pub fn new(yearly_mean_return: f64, yearly_volatility: f64) -> Self {
        Self {
            mean_return: yearly_mean_return / 12.0f64,
            volatility: yearly_volatility / 12.0f64,
            current_return: yearly_mean_return / 12.0f64, // Start at the mean
        }
    }

    pub fn monthly_returns(&mut self) -> MeanReversionIterator {
        let normal_dist = Normal::new(0.0, self.volatility).unwrap();
        MeanReversionIterator {
            model: self,
            normal_dist,
        }
    }

}

pub struct MeanReversionIterator<'a> {
    model: &'a mut MeanReversion,
    normal_dist: Normal<f64>,
}

impl<'a> Iterator for MeanReversionIterator<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let shock = self.normal_dist.sample(&mut rand::rng());
        let adjustment = 0.1 * (self.model.mean_return - self.model.current_return);
        self.model.current_return += adjustment + shock;
        Some(self.model.current_return)
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn interests() {
        let mut mean_reversion = MeanReversion::new(0.05, 0.1);
        let mut mean_reversion_iterator = mean_reversion.monthly_returns();
        let mut portfolio_value = 50000.0;
        for _ in 0..40*12 {
            portfolio_value += 500.0;
            let monthly_return =  mean_reversion_iterator.next().unwrap();
            portfolio_value *= 1. + monthly_return;
            println!("Value: {:?}", portfolio_value);
        }
    }

}