use cli_table::format::Justify;
use cli_table::{Cell, Style, Table};
use rusty_money::iso::Currency;
use rusty_money::{iso, Money};

struct Player {
    index: usize,
    number: usize,
    original: String,
    allocated: Option<String>,
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
    fn get_index(&self) -> usize {
        self.index
    }

    /// get number (no.) of player
    fn get_number(&self) -> usize {
        self.number
    }

    /// get original money
    fn get_original(&self) -> String {
        self.original.clone()
    }

    /// get allocated amount
    fn get_allocated(&self) -> String {
        self.allocated.to_owned().unwrap()
    }

    /// update allocated result to player
    fn set_allocated(&mut self, money: Money<Currency>) {
        self.allocated = Some(money.amount().to_string())
    }

    /// get player's name or number (if name not provided)
    fn get_player_name_or_number(&self) -> String {
        match self.name.as_ref() {
            Some(name) => name.to_owned(),
            None => self.number.to_string(),
        }
    }
}

pub struct Round<'a> {
    total: Money<'a, Currency>,
    players: Vec<Player>,
    display_format: Format,
    result: Option<Vec<Money<'a, Currency>>>,
}

impl Round<'_> {
    pub fn new(input: &[String]) -> Self {
        let mut players = vec![];
        let buy_amount = &input[1..];
        for (i, item) in buy_amount.iter().enumerate() {
            players.push(Player::new(i, item.to_owned()));
        }
        Self {
            total: Money::from_str(&input[0], iso::TWD).unwrap(),
            players,
            display_format: Format::Table,
            result: None,
        }
    }

    /// Start allocating money
    pub fn allocate(&mut self) -> &Round<'_> {
        // get the allocated result and update to field
        self.result = Some(self.total.allocate(self.get_ratios()).unwrap());
        // update result to each player struct
        self.players.iter_mut().for_each(|p| p.set_allocated(self.result.as_ref().unwrap()[p.get_index()]));

        self
    }

    /// Get ratios, price of each player bought
    fn get_ratios(&self) -> Vec<i32> {
        let mut ratios = Vec::new();
        self.players
            .iter()
            .for_each(|x| ratios.push(x.original.parse::<i32>().unwrap()));
        ratios
    }

    /// Display result
    pub fn display(&self) {
        println!("Total to be allocated: {}", self.total);
        match self.display_format {
            Format::Table => self.display_table(),
        }
    }

    /// display with table format
    fn display_table(&self) {
        let mut table = Vec::new();
        self.players.iter().for_each(|p| {
            table.push(vec![
                p.get_player_name_or_number().cell(),
                p.get_original().cell().justify(Justify::Right),
                self.result
                    .as_ref()
                    .unwrap()
                    .get(p.get_index())
                    .unwrap()
                    .cell()
                    .justify(Justify::Right),
            ])
        });

        // print result table
        println!(
            "{}",
            table
                .table()
                .title(vec![
                    "No./Name".cell().bold(true),
                    "Original".cell().bold(true),
                    "Allocated".cell().bold(true)
                ])
                .display()
                .unwrap()
        );
    }
}

/// The format option of displaying result
pub enum Format {
    Table,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_ratios() {
        // init input example from README
        let args = vec![String::from("100"), String::from("40"), String::from("70")];
        let input: &[String] = &args[..];

        assert_eq!(Round::new(input).get_ratios(), vec![40, 70]);
    }

    #[test]
    fn test_allocate() {
        // init input example from README
        let args = vec![String::from("100"), String::from("40"), String::from("70")];
        let input: &[String] = &args[..];

        assert_eq!(
            Round::new(input).allocate().result.as_ref().unwrap(),
            &vec![
                Money::from_str("36", iso::TWD).unwrap(),
                Money::from_str("64", iso::TWD).unwrap(),
            ]
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
