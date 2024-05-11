use cli_table::format::Justify;
use cli_table::{Cell, Style, Table};

/// Print result on terminal
pub fn table(input: &[String]) {
    let allocated = crate::abacus::calculate(input);

    // print result with origin price
    let mut table = vec![];
    for i in 1..input.len() {
        // i => origin order from input
        // &input[i] => origin input value
        table.push(vec![
            i.cell(),
            (&input[i]).cell().justify(Justify::Right),
            allocated
                .get(i - 1)
                .unwrap()
                .amount()
                .cell()
                .justify(Justify::Right),
        ]);
    }
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
