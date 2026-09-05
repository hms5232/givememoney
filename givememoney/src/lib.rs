use money::Money;
use std::str::FromStr;

/// A participant in a money allocation round.
pub struct Player {
    index: usize,
    number: usize,
    original: String,
    allocated: Option<u32>,
    name: Option<String>,
}

impl Player {
    fn new(index: usize, input: String) -> Self {
        let mut name = None;
        let mut original = input;
        if original.contains('=') {
            let split: Vec<&str> = original.split('=').collect();
            name = Some(split[0].to_string());
            original = split[1].to_string();
        }
        Self {
            index,
            number: index + 1,
            original,
            allocated: None,
            name,
        }
    }

    /// get player index of ratios
    pub fn index(&self) -> usize {
        self.index
    }

    /// get number (no.) of the player
    fn number(&self) -> usize {
        self.number
    }

    /// get original money
    pub fn original(&self) -> String {
        self.original.clone()
    }

    /// get allocated amount
    fn allocated(&self) -> String {
        self.allocated.unwrap().to_string()
    }

    /// update result of allocated to player
    fn set_allocated(&mut self, money: u32) {
        self.allocated = Some(money)
    }

    /// get player's name or number (if name not provided)
    pub fn get_player_name_or_number(&self) -> String {
        match self.name.as_ref() {
            Some(name) => name.to_owned(),
            None => self.number.to_string(),
        }
    }
}

/// A single round of money allocation.
pub struct Round {
    total: Money,
    players: Vec<Player>,
    result: Option<Vec<u32>>,
}

impl Round {
    /// Create a new round from CLI arguments.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use givememoney::Round;
    ///
    /// let input = [
    ///     String::from("100"),
    ///     String::from("40"),
    ///     String::from("Alice=70"),
    /// ];
    /// let mut round = Round::new(&input);
    /// round.allocate();
    ///
    /// assert_eq!(round.total().amount(), 100);
    /// assert_eq!(round.result(), Some(&[36, 64][..]));
    /// ```
    pub fn new(input: &[String]) -> Self {
        let mut players = vec![];
        let buy_amount = &input[1..];
        for (i, item) in buy_amount.iter().enumerate() {
            players.push(Player::new(i, item.to_owned()));
        }
        Self {
            total: Money::from_str(&input[0]).unwrap(),
            players,
            result: None,
        }
    }

    /// Allocate money and fill result into self and each player field.
    pub fn allocate(&mut self) -> &Round {
        // get the allocated result and update to field
        self.result = Some(self.total.allocate(self.ratios()).unwrap());
        // update result to each player struct
        self.players
            .iter_mut()
            .for_each(|p| p.set_allocated(self.result.as_ref().unwrap()[p.index()]));

        self
    }

    /// Get ratios, price of each player bought
    fn ratios(&self) -> Vec<u32> {
        let mut ratios = Vec::new();
        self.players
            .iter()
            .for_each(|x| ratios.push(x.original.parse::<u32>().unwrap()));
        ratios
    }

    /// Get total
    pub fn total(&self) -> Money {
        self.total
    }

    /// Get all players
    pub fn players(&self) -> &[Player] {
        self.players.as_slice()
    }

    /// Get allocation result
    ///
    /// `None` if [allocate()] has not been called yet.
    pub fn result(&self) -> Option<&[u32]> {
        self.result.as_deref()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_ratios() {
        // init input example from README
        let args = vec![String::from("100"), String::from("40"), String::from("70")];
        let input: &[String] = &args[..];

        assert_eq!(Round::new(input).ratios(), vec![40, 70]);
    }

    #[test]
    fn test_allocate() {
        // init input example from README
        let args = vec![String::from("100"), String::from("40"), String::from("70")];
        let input: &[String] = &args[..];

        assert_eq!(
            Round::new(input).allocate().result.as_ref().unwrap(),
            &vec![36, 64]
        );
    }

    #[test]
    fn test_unnamed_player() {
        // init input
        let args = vec![String::from("100"), String::from("40"), String::from("70")];
        let input: &[String] = &args[..];

        let mut displayed_name = Vec::new();
        Round::new(input)
            .players
            .iter()
            .for_each(|p| displayed_name.push(p.get_player_name_or_number()));
        // all players are anonymous, so give them a number (as input order)
        assert_eq!(displayed_name, vec!["1", "2"]);
    }

    #[test]
    fn test_named_player() {
        // init input
        let args = vec![
            String::from("100"),
            String::from("40"),
            String::from("Alice=70"),
        ];
        let input: &[String] = &args[..];

        let mut displayed_name = Vec::new();
        Round::new(input)
            .players
            .iter()
            .for_each(|p| displayed_name.push(p.get_player_name_or_number()));
        // one player is named as "Alice" at second input,
        // so first is number and second is given name.
        assert_eq!(displayed_name, vec!["1", "Alice"]);
    }
}
