#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tick {
	pub symbol: String,
	pub bid: f64,
	pub ask: f64,
	pub price: f64,
	pub timestamp: u64,
}