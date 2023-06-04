use std::{path::Path, str::from_utf8, fs::read};
use serde::Deserialize;
use toml::{self, Table};

#[derive(Deserialize)]
pub struct Data {
	pub metadata: Metainfo,
	pub downloads: Table,
}

#[derive(Deserialize)]
pub struct Metainfo {
	pub name: String,
	pub description: String,
	pub version: String,
	pub provides: Vec<String>,
	pub deps: Vec<String>
}

pub struct Metadata {
	path: String,
}

impl Metadata {
	pub fn new(path: String) -> Self {
		Self {
			path
		}
	}

	fn read_content(&self) -> Result<String, String> {
		let path = Path::new(&self.path);
		if !path.is_file() {
			return Err(String::from("Given path isn't a file!"));
		}

		Ok(match from_utf8(&read(path).unwrap()) {
			Ok(v) => String::from(v),
			Err(e) => panic!("Cannot read file! invalid UTF-8 sequence: {}", e),
		})
	}

	pub fn parse(&self) -> Result<Data, toml::de::Error> {
		toml::from_str(self.read_content().unwrap().as_str())
	}
}