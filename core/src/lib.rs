use money::Money;
use std::str::FromStr;

/// A participant in a money allocation round.
pub struct Player {
    index: usize,
    number: usize,
    original: u32,
    allocated: Option<u32>,
    name: Option<String>,
}

impl Player {
    fn new(index: usize, input: String) -> Result<Self, Error> {
        let (name, amount) = match input.split_once('=') {
            Some((n, a)) => (Some(n.to_string()), a),
            None => (None, input.as_str()),
        };
        let original: u32 = amount.parse::<u32>().map_err(|_| Error::InvalidNumber {
            position: index,
            value: amount.to_string(),
            name: name.clone(),
        })?;
        Ok(Self {
            index,
            number: index + 1,
            original,
            allocated: None,
            name,
        })
    }

    /// get player index of ratios
    pub fn index(&self) -> usize {
        self.index
    }

    /// get number (no.) of the player
    pub fn number(&self) -> usize {
        self.number
    }

    /// get original number
    pub fn original(&self) -> u32 {
        self.original
    }

    /// get allocated amount
    pub fn allocated(&self) -> Result<u32, Error> {
        match self.allocated {
            Some(allocated) => Ok(allocated),
            None => Err(Error::Unallocated),
        }
    }

    /// Get player name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// update result of allocated to player
    fn set_allocated(&mut self, money: u32) {
        self.allocated = Some(money)
    }

    /// get player's name or number (if name not provided)
    pub fn get_player_name_or_number(&self) -> String {
        match self.name() {
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
    /// let mut round = Round::new(&input).unwrap();
    /// round.allocate().unwrap();
    ///
    /// assert_eq!(round.total().amount(), 100);
    /// assert_eq!(round.result(), Some(&[36, 64][..]));
    /// ```
    pub fn new(input: &[String]) -> Result<Self, Error> {
        let total = match Money::from_str(input.first().ok_or(Error::EmptyInput)?) {
            Ok(total) => total,
            Err(_) => return Err(Error::MoneyError),
        };
        let mut players = vec![];
        let buy_amount = &input[1..];
        for (i, item) in buy_amount.iter().enumerate() {
            let player = Player::new(i, item.to_owned())?;
            players.push(player)
        }
        Ok(Self {
            total,
            players,
            result: None,
        })
    }

    /// Allocate money and fill result into self and each player field.
    pub fn allocate(&mut self) -> Result<&Round, Error> {
        // get the allocated result and update to field
        let result = self
            .total
            .allocate(self.ratios())
            .map_err(|_| Error::MoneyError)?;
        // update result to each player struct
        self.players
            .iter_mut()
            .for_each(|p| p.set_allocated(result[p.index()]));

        self.result = Some(result);
        Ok(self)
    }

    /// Get ratios, price of each player bought
    fn ratios(&self) -> Vec<u32> {
        let mut ratios = Vec::new();
        self.players.iter().for_each(|x| ratios.push(x.original));
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

#[derive(Debug)]
pub enum Error {
    /// Error from [`Money`].
    MoneyError,
    InvalidNumber {
        position: usize,
        value: String,
        name: Option<String>,
    },
    /// The round is waiting for allocation.
    Unallocated,
    EmptyInput,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_ratios() {
        // init input example from README
        let args = vec![String::from("100"), String::from("40"), String::from("70")];
        let input: &[String] = &args[..];

        assert_eq!(Round::new(input).unwrap().ratios(), vec![40, 70]);
    }

    #[test]
    fn test_allocate() {
        // init input example from README
        let args = vec![String::from("100"), String::from("40"), String::from("70")];
        let input: &[String] = &args[..];

        assert_eq!(
            Round::new(input)
                .unwrap()
                .allocate()
                .unwrap()
                .result
                .as_ref()
                .unwrap(),
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
            .unwrap()
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
            .unwrap()
            .players
            .iter()
            .for_each(|p| displayed_name.push(p.get_player_name_or_number()));
        // one player is named as "Alice" at second input,
        // so first is number and second is given name.
        assert_eq!(displayed_name, vec!["1", "Alice"]);
    }
}
