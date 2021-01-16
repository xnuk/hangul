pub trait Key {
	fn has_shift(&self) -> bool { false }
	fn has_alt(&self) -> bool { false }
	fn has_ctrl(&self) -> bool { false }
}

#[derive(Debug)]
pub enum WritingEvent {
	Commit(String, Option<String>),
	Display(String),
	Cancel,
	None,
}

pub trait KeyInputHandler<K: Key> {
	type Candidates: IntoIterator<Item = String>;

	fn keydown(&mut self, _key: &K) -> WritingEvent { WritingEvent::None }
	fn keyup  (&mut self, _key: &K) -> WritingEvent { WritingEvent::None }

	fn candidates(&self) -> Option<&Self::Candidates> { None }
	fn select_candidate(&mut self, _item: &String) -> WritingEvent {
		WritingEvent::None
	}
}

pub struct QWERTY(pub u8);

impl Key for QWERTY {
	fn has_shift(&self) -> bool {
		let QWERTY(code) = *self;
		match code {
			| u8::MIN..=31
			| b' '
			| b'\''
			| b','
			| b'-'
			| b'.'
			| b'/'
			| b'0'..=b'9'
			| b';'
			| b'='
			| b'[' | b']'
			| b'\\'
			| b'`'
			| b'a'..=b'z'
			=> false,

			| 127..=u8::MAX
			| b'A'..=b'Z'
			| b'!'..=b'+'
			| b':'
			| b'<' | b'>'
			| b'{' | b'}'
			| b'?' | b'@'
			| b'|' | b'~'
			| b'^' | b'_'
			=> true,
		}
	}
}

#[derive(Default, Debug)]
pub struct StateTest {
	candidates: Vec<String>,
	compositing: String,
}

impl KeyInputHandler<QWERTY> for StateTest {
	type Candidates = Vec<String>;

	fn keydown(&mut self, key: &QWERTY) -> WritingEvent {
		let QWERTY(code) = key;
		self.candidates = match code {
			b'A' => vec![
				"Apple".to_string(),
				"Banana".to_string(),
				"Kiwi".to_string(),
			],

			b'a' => vec![
				"anal sex".to_string(),
				"oral sex".to_string(),
				"prostate massage".to_string(),
			],

			_ => vec![]
		};

		if self.candidates.len() > 0 {
			self.compositing += "s";
			WritingEvent::Display(self.compositing.clone())
		} else {
			let cl = self.compositing.clone();
			self.compositing = String::from(char::from(*code));
			WritingEvent::Commit(cl, Some(self.compositing.clone()))
		}
	}

	fn candidates(&self) -> Option<&Vec<String>> {
		if self.candidates.len() > 0 {
			Some(&self.candidates)
		} else {
			None
		}
	}

	fn select_candidate(&mut self, item: &String) -> WritingEvent {
		self.compositing.clear();
		WritingEvent::Commit(item.clone(), None)
	}
}
