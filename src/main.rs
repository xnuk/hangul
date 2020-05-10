use std::io;

fn something(line: &str) {
	let result = hangeul::decompose(&line);
	println!("{:?}", result)
}

fn main() -> io::Result<()> {
	let mut line = String::new();
	io::stdin().read_line(&mut line)?;
	let line = line.trim_end_matches::<&[char]>(&['\r', '\n']);
	something(&line);
	Ok(())
}
