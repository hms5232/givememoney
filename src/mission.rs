use cli_table::format::Justify;
use cli_table::{Cell, Style, Table};
use rusty_money::iso::Currency;
use rusty_money::{iso, Money};

struct Player<'a> {
    index: usize,
    number: usize,
    original: String,
    allocated: Option<Money<'a, Currency>>,
    name: Option<String>,
}

impl Player<'_> {
    fn new(index: usize, original: String) -> Self {
        Self {
            index,
            number: index + 1,
            original,
            allocated: None,
            name: None,
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
    fn get_allocated(&self) -> Money<Currency> {
        self.allocated.unwrap()
    }

    // /// update allocated result to player
    // fn set_allocated(&mut self, money: Money<Currency>) {
    //     todo!("FIXME: assignment requires that `'1` must outlive `'2`");
    //     self.allocated = Some(money)
    // }
}

pub struct Round<'a> {
    total: Money<'a, Currency>,
    players: Vec<Player<'a>>,
    display_format: Format,
    result: Option<Vec<Money<'a, Currency>>>,
}

impl Round<'_> {
    pub fn new(input: &[String]) -> Self {
        let mut players = vec![];
        let buy_amount = &input[1..];
        for i in 0..buy_amount.len() {
            players.push(Player::new(i, buy_amount[i].to_owned()));
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
        // TODO: update result to each player struct
        // self.players.iter_mut().for_each(|p| p.set_allocated(self.result.as_ref().unwrap()[p.get_index()]));

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
        match self.display_format {
            Format::Table => self.display_table(),
        }
    }

    /// display with table format
    fn display_table(&self) {
        let mut table = Vec::new();
        self.players.iter().for_each(|p| {
            table.push(vec![
                p.get_number().cell(),
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
                    "No.".cell().bold(true),
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
