//! Simple money implementation
//!
//! Inspired by https://github.com/varunsrin/rusty_money/pull/104

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
            let max = *fractions.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
            let index = fractions.iter().position(|&r| r == max).unwrap();
            allocations[index] += 1;
            remainder -= 1;
            fractions[index] = 0.0;
        }
        Ok(allocations)
    }

    pub(crate) fn from_str(p0: &str) -> Result<Money, &str> {
        let amount = p0.parse::<u32>().unwrap();
        Ok(Money::new(amount))
    }
}
