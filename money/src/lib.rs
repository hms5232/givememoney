//! Simple money implementation
//!
//! Inspired by <https://github.com/varunsrin/rusty_money/pull/104>

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Money {
    amount: u32,
}

impl Money {
    pub fn new(amount: u32) -> Self {
        Self { amount }
    }

    /// Get amount
    pub fn amount(&self) -> u32 {
        self.amount
    }

    /// Allocate money fairly by ratios
    pub fn allocate(&self, ratios: Vec<u32>) -> Result<Vec<u32>, &str> {
        if ratios.is_empty() {
            return Err("Ratios cannot be empty");
        }

        let mut remainder = self.amount;
        let ratio_total = ratios.iter().sum::<u32>();

        let mut allocations: Vec<u32> = Vec::new();
        let mut fractions: Vec<f32> = Vec::new();

        for ratio in ratios {
            if ratio == 0 {
                return Err("Ratio cannot be 0");
            }

            let share = self.amount * ratio / ratio_total;

            fractions.push((self.amount as f32 * ratio as f32 / ratio_total as f32) - share as f32);
            allocations.push(share);
            remainder -= share;
        }

        // allocate the remainder to the player with the largest fraction
        while remainder > 0 {
            let max = match fractions.iter().max_by(|a, b| a.total_cmp(b)) {
                Some(max) => *max,
                None => return Err("Failed to find max fraction when allocating remainder"),
            };
            let index = match fractions.iter().position(|&r| r == max) {
                Some(index) => index,
                None => {
                    return Err("Failed to find max index of fraction when allocating remainder");
                }
            };
            allocations[index] += 1;
            remainder -= 1;
            fractions[index] = 0.0;
        }
        Ok(allocations)
    }
}

impl FromStr for Money {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(amount) => Ok(Money::new(amount)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Money;
    use std::str::FromStr;

    #[test]
    fn test_allocate() {
        assert_eq!(Money::new(100).allocate(vec![40, 70]), Ok(vec![36, 64]));
        assert_eq!(
            Money::new(100).allocate(vec![30, 20, 40, 50]),
            Ok(vec![21, 14, 29, 36])
        );
        assert_eq!(
            Money::new(100).allocate(vec![0, 100]),
            Err("Ratio cannot be 0")
        );
        assert_eq!(
            Money::new(100).allocate(vec![]),
            Err("Ratios cannot be empty")
        );
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Money::from_str("100"), Ok(Money::new(100)));
    }

    #[test]
    fn test_from_str_err() {
        assert!(Money::from_str("a12").is_err());
    }
}
