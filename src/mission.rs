use cli_table::format::Justify;
use cli_table::{Cell, Style, Table};
use givememoney::Round;

pub(crate) struct Mission {
    round: Round,
}

impl Mission {
    pub fn start(input: &[String]) -> Self {
        let mut round = Round::new(input);
        round.allocate();
        Self { round }
    }

    /// Display result
    pub fn display(&self) {
        println!("Total to be allocated: {}", self.round.total().amount());
        self.display_table()
    }

    /// display with table format
    fn display_table(&self) {
        let mut table = Vec::new();
        self.round.players().iter().for_each(|p| {
            table.push(vec![
                p.get_player_name_or_number().cell(),
                p.original().cell().justify(Justify::Right),
                self.round
                    .result()
                    .map(|r| r[p.index()])
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
