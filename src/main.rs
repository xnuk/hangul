use std::io;

use prostate::layout::{Key, KeyInputHandler, QWERTY, StateTest};

// fn something(line: &str) {
// 	let result = hangeul::decompose(&line);
// 	println!("{:?}", result)
// }

fn main() -> io::Result<()> {
	// let mut line = String::new();
	// io::stdin().read_line(&mut line)?;
	// let line = line.trim_end_matches::<&[char]>(&['\r', '\n']);
	// something(&line);

	println!("{:?}", QWERTY(b'A').has_shift());

	let mut st: StateTest = Default::default();
	println!("{:?}", &st);

	println!("{:?}\t\t{:?}", st.keydown(&QWERTY(b'B')), &st);
	println!("{:?}\t\t{:?}", st.keydown(&QWERTY(b'A')), &st);
	println!("{:?}\t\t{:?}", st.keydown(&QWERTY(b'N')), &st);
	println!("{:?}\t\t{:?}", st.keydown(&QWERTY(b'a')), &st);
	println!("{:?}\t\t{:?}", st.keydown(&QWERTY(b'n')), &st);
	println!("{:?}\t\t{:?}", st.select_candidate(&"prostate".to_string()), &st);

	Ok(())
}
