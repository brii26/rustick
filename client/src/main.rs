#[derive(Debug)]
struct Test {
	name: String,
	age: u32
}

fn main() {
	let new_test = Test
		{
			name: String::from("test"),
			age: 1,
		};
    println!("{:?}", new_test);
}
