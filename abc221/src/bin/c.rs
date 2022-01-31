#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
	let s = read.chars();
	let mut counts = vec![0;10];
	let mut ans = 0;
	for i in 0..s.len() {
		counts[(s[i] as u8 - '0' as u8) as usize] += 1;
	}
	for i in 0..(1 << s.len()) {
		let mut a = Vec::new();
		let mut b = Vec::new();
		for j in 0..s.len() {
			if (i & (1 << j) != 0) {
				a.push(s[j]);
			}else {
				b.push(s[j]);
			}
		}
		a.sort_by_key(|&x| std::cmp::Reverse(x));
		b.sort_by_key(|&x| std::cmp::Reverse(x));
		let mut mula = 0;
		let mut mulb = 0;
		for j in 0..a.len() {
			mula *= 10;
			mula += (a[j] as u8 - '0' as u8) as i128;
		}
		for j in 0..b.len() {
			mulb *= 10;
			mulb += (b[j] as u8 - '0' as u8) as i128;
		}
		ans = std::cmp::max(ans,mula * mulb);
	}
	println!("{}",ans);
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

const INF:i64 = 1i64 << 60;
