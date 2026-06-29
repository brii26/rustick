use rand::RngExt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tick {
	symbol: String,
	bid: f64,
	ask: f64,
	price: f64,
	timestamp: u64
}

const SYMBOLS: &[&str] = &["BBCA", "BBRI", "TLKM", "BMRI", "ASII"];

pub fn generate_tick() -> Tick {
	let mut rng = rand::rng();
	
	// random symbol
	let sym_len = SYMBOLS.len();
	let sym_idx = rng.random_range(0..sym_len);
	
	// random price
	let price = rng.random_range(1000.0..15000.0);
	
	// random bid
	let bid = price - rng.random_range(0.0..50.0);
	
	// random ask 
	let ask = price + rng.random_range(0.0..50.0);
	
	// random timestamp
	let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;

	Tick {
		symbol: SYMBOLS[sym_idx].to_string(),
		bid,
		ask,
		price,
		timestamp
	}
}