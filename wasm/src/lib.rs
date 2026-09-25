//! WebAssembly wrapper for givememoney

use givememoney::Round;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Data format received from JavaScript.
#[derive(Deserialize)]
struct RoundRequest {
    total: u32,
    players: Vec<PlayerInput>,
}

#[derive(Deserialize)]
struct PlayerInput {
    name: Option<String>,
    amount: u32,
}

/// Allocated result
#[derive(Serialize)]
struct RoundResponse {
    total: u32,
    players: Vec<PlayerOutput>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PlayerOutput {
    name: Option<String>,
    /// No.
    number: usize,
    /// Original money
    original: u32,
    /// Allocated money
    allocated: u32,
    /// The name to be shown
    display_name: String,
}

/// Allocate the total money to players by their ratios.
///
/// # Usage
///
/// `data` should be an object like:
///
/// ```js
/// {
///   total: 100,
///   players: [
///     { name: "Alice", amount: 70 },
///     { amount: 40 }, // [name](cci:1://file:///R:/givememoney/core/src/lib.rs:11:4-14:5) is optional
///   ],
/// }
/// ```
///
/// Returns an object like:
///
/// ```js
/// {
///   total: 100,
///   players: [
///     { name: "Alice", number: 1, original: 70, allocated: 64, displayName: "Alice" },
///     { name: undefined, number: 2, original: 40, allocated: 36, displayName: "2" },
///   ],
/// }
/// ```
///
/// # Errors
///
/// Throws an `Error` if:
///
/// - `data` doesn't match the shape above (e.g. `amount` is not a non-negative integer)
/// - `players` is empty
/// - any `amount` is 0
#[wasm_bindgen]
pub fn allocate(data: JsValue) -> Result<JsValue, JsError> {
    // parse input from js
    let request: RoundRequest = serde_wasm_bindgen::from_value(data)?;

    // make cli style input
    let mut input = vec![request.total.to_string()];
    input.extend(request.players.iter().map(|p| match &p.name {
        Some(name) => format!("{name}={}", p.amount), // cli format
        None => p.amount.to_string(),
    }));

    let mut round = Round::new(&input).map_err(|e| JsError::new(&format!("{e:?}")))?;
    round
        .allocate()
        .map_err(|e| JsError::new(&format!("{e:?}")))?;

    Ok(serde_wasm_bindgen::to_value(&RoundResponse {
        total: request.total,
        players: round
            .players()
            .iter()
            .map(|player| {
                Ok(PlayerOutput {
                    name: player.name().map(str::to_owned),
                    number: player.number(),
                    original: player.original(),
                    allocated: player
                        .allocated()
                        .map_err(|e| JsError::new(&format!("{e:?}")))?,
                    display_name: player.get_player_name_or_number(),
                })
            })
            .collect::<Result<Vec<_>, JsError>>()?,
    })?)
}
