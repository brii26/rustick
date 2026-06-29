mod generator;
use generator::generate_tick;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	// set interval 1ms
	let mut interval = tokio::time::interval(
		std::time::Duration::from_millis(100)
	);

	// infinite loop generate_tick every 1 ms
	loop {
		interval.tick().await;
		let tick = generate_tick();
		println!("{:?}", tick);
	}
}