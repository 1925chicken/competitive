#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
	let h = read.usize();
	let w = read.usize();
	let n = read.usize();
	let mut rca:Vec<_> = (0..n).map(|i| (read.usize() - 1,read.usize() - 1,read.i64(),i)).collect();
	let mut dp = vec![0;n];
	let mut r = vec![0;h];
	let mut c = vec![0;w];
	rca.sort_by_key(|&x| std::cmp::Reverse(x.2));
	//println!("{:?}",rca);
	let mut i = 0;
	while i < n {
		//println!("{}",i);
		let mut j = i;
		while j < n && rca[i].2 == rca[j].2 {
			dp[rca[j].3] = std::cmp::max(r[rca[j].0],c[rca[j].1]);
			j += 1;
		}
		for k in i..std::cmp::min(n,j) {
			r[rca[k].0] = std::cmp::max(r[rca[k].0], dp[rca[k].3] + 1);
			c[rca[k].1] = std::cmp::max(c[rca[k].1], dp[rca[k].3] + 1);
		}
		i = j;
		//println!("{:?}",dp);
	}
	for val in dp {
		println!("{}",val);
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

const INF:i64 = 1i64 << 60;
