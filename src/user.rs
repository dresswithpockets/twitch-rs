
pub struct User {
	id: u64,
	display_name: String
}

impl User {
	pub fn id(&self) -> &u64 {
		&self.id
	}

	pub fn display_name(&self) -> &String {
		&self.display_name
	}
}
