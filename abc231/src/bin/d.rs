#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
	let n = read.usize();
	let m = read.usize();
	let mut rec = vec![0;n];
	let mut uf = UnionFind::new(n);
	for _ in 0..m {
		let a = read.usize() - 1;
		let b = read.usize() - 1;
		rec[a] += 1;
		rec[b] += 1;
		if (uf.root(a) == uf.root(b)) || rec[a] > 2 || rec[b] > 2 {
			println!("No");
			return;
		}
		uf.merge(a,b);
	}
	println!("Yes");
}

use std::mem::*;
struct UnionFind {
	par: Vec<i64>,
}
#[allow(dead_code)] 
impl UnionFind {
	pub fn new(n: usize) -> Self {
		let par = (0..n).map(|_| -1).collect();
		Self{par}
	}
	pub fn root(&mut self, x: usize) -> i64 {
	  if self.par[x] < 0{
		  x as i64
	  }else{
		  self.par[x] = self.root(self.par[x] as usize);
		  self.par[x]
	  }
	}
	pub fn same(&mut self, x: usize, y: usize) -> bool {
		self.root(x) == self.root(y)
	}
 
	pub fn merge(&mut self, x: usize, y: usize) -> bool {
		let mut _x: i64 = self.root(x);
		let mut _y: i64 = self.root(y);
		if _x == _y {
			return false;
		}
		if self.par[_x as usize] > self.par[_y as usize] {
			swap(&mut _x, &mut _y);
		}
		self.par[_x as usize] += self.par[_y as usize];
		self.par[_y as usize] = _x as i64;
		 return true;
	}
	pub fn size(&mut self, x: usize) -> i64 {
		(self.par[x]) as i64 * -1
	}
}

//use proconio::input;
fn main() {
	let t = std::io::stdin();
	let mut read = snio::Reader::new(t.lock());
	let n = 1;
	for _ in 0..n {
		solve(&mut read);
	}
}
 
#[allow(dead_code)]
pub mod snio {
	pub struct Reader<R: std::io::BufRead> {
		reader: R,
		buf: std::collections::VecDeque<String>,
	}
 
	impl<R: std::io::BufRead> Reader<R> {
		pub fn new(reader: R) -> Self {
			Self {
				reader,
				buf: std::collections::VecDeque::new(),
			}
		}
		fn load(&mut self) {
			while self.buf.is_empty() {
				let mut s = String::new();
				let length = self.reader.read_line(&mut s).unwrap();
				if length == 0 {
					break;
				}
				self.buf.extend(s.split_whitespace().map(|s| s.to_owned()));
			}
		}
		pub fn string(&mut self) -> String {
			self.load();
			self.buf.pop_front().unwrap_or_else(|| panic!("input ended"))
		}
		pub fn char(&mut self) -> char {
			let string = self.string();
			let mut chars = string.chars();
			let res = chars.next().unwrap();
			assert!(chars.next().is_none(), "invalid input!");
			res
		}
		pub fn chars(&mut self) -> Vec<char> {
			self.read::<String>().chars().collect()
		}
		pub fn read<T: std::str::FromStr>(&mut self) -> T
			where
				<T as ::std::str::FromStr>::Err: ::std::fmt::Debug,
		{
			self.string().parse::<T>().expect("Failed to parse the input.")
		}
	}
	macro_rules! definition_of_reader_of_numbers {
			($($ty:tt,)*) => {
				impl <R:std::io::BufRead> Reader<R> {
					$(
					#[inline]
					pub fn $ty (&mut self) -> $ty {
						self.read::<$ty>()
					}
					)*
				}
			}
		}
	definition_of_reader_of_numbers! {
		u8,u16,u32,u64,usize,
		i8,i16,i32,i64,isize,
		f32,f64,
	}
}

