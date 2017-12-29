//! A simple, functional file IO mini-library

use std::iter::Iterator;
use std::io::{Result, Read};

pub trait SimpleIO {
	fn read_string(&mut self) -> Result<String>;
	fn read_chars(&mut self) -> Result<Vec<char>>;
	fn read_lines(&mut self) -> Result<Vec<String>>;
	fn read_vec(&mut self) -> Result<Vec<u8>>;
}

pub struct Chars {
	index: usize,
	inner: Vec<char>
}

impl Iterator for Chars {
	type Item = char;

	fn next(&mut self) -> Option<char> {
		self.index += 1;
		self.inner.get(self.index)
			.map(|x| *x)
	}
}

impl Chars {
	pub fn first(&self) -> Option<char> {
		self.inner.get(0)
			.map(|x| *x)
	}

	pub fn get(&self, idx: usize) -> Option<char> {
		self.inner.get(idx)
			.map(|x| *x)
	}

	pub fn from(src: Vec<char>) -> Chars {
		Chars {
			index: 0,
			inner: src,
		}
	}
}

pub struct Lines {
	index: usize,
	inner: Vec<String>,
}

impl Iterator for Lines {
	type Item = String;

	fn next(&mut self) -> Option<String> {
		self.index += 1;
		self.inner.get(self.index)
			.map(|x| x.clone())
	}
}

impl Lines {
	pub fn first(&self) -> Option<String> {
		self.inner.get(0)
			.map(|x| x.clone())
	}

	pub fn get(&self, idx: usize) -> Option<String> {
		self.inner.get(idx)
			.map(|x| x.clone())
	}

	pub fn from(src: Vec<String>) -> Lines {
		Lines {
			index: 0,
			inner: src,
		}
	}
}

impl<T> SimpleIO for T
	where T: Read
{
	fn read_string(&mut self) -> Result<String> {
		let mut buf = String::new();
		self.read_to_string(&mut buf)
			.map(|_| buf)
	}

	fn read_chars(&mut self) -> Result<Vec<char>> {
		let mut buf = String::new();
		self.read_to_string(&mut buf)
			.map(|_| buf.chars().collect())
	}

	fn read_lines(&mut self) -> Result<Vec<String>> {
		let mut buf = String::new();
		self.read_to_string(&mut buf)
			.map(|_| buf.lines().map(|x| x.to_string()).collect())
	}

	fn read_vec(&mut self) -> Result<Vec<u8>> {
		let mut buf = Vec::new();
		self.read_to_end(&mut buf)
			.map(|_| buf)
	}
}
