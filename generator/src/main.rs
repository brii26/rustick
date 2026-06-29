mod generator;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	// set interval 100ms
	let mut interval = tokio::time::interval(
		std::time::Duration::from_millis(100)
	);

	// infinite loop generate_tick every 100 ms
	let socket = UdpSocket::bind("0.0.0.0:0").await?;
	loop {
		interval.tick().await;
		let tick = generator::generate_tick();
		generator::send_tick(&socket, &tick).await?;
		println!("{:?}", tick);
	}
}