#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
	let n = read.i64();
	let a = read.i64();
	let b = read.i64();
	let p = read.usize();
	let q = read.usize();
	let r = read.usize();
	let s = read.usize();
	let mut lb = std::cmp::max(1 - a,1 - b);
	let mut ub = std::cmp::min(n - a,n - b);
	let mut grid = vec![vec!['.';s - r + 1];q - p + 1];
	let mut ak_min = a + lb;
	let mut ak_max = a + ub;
	let mut bk_min = b + lb;
	let mut bk_max = b + ub;
	let mut piv_p = 0;
	let mut piv_r = 0;
	for i in 0..q - p + 1 {
		for j in 0..s - r + 1 {
			if ak_min == (p + i) as i64 {
				piv_p = i;
			}
			if bk_min == (r + j) as i64 {
				piv_r = j;
			}
		}
 	}//println!("OK");
	for i in 0..(ub - lb + 1) as usize {
		if i + piv_p < q - p + 1 && i + piv_r < s - r + 1{
		grid[i + piv_p][i + piv_r] = '*';
		}
	}	
	let mut lb = std::cmp::max(1 - a,b - n);
	let mut ub = std::cmp::min(n - a,b - 1);
	let mut ak_min = a + lb;
	let mut ak_max = a + ub;
	let mut bk_min = b + lb;
	let mut bk_max = b + ub;
	let mut piv_p = 0;
	let mut piv_r = 0;
	for i in 0..q - p + 1 {
		for j in r..=s{
			if ak_min == (p + i) as i64 {
				piv_p = i;
			}
			if bk_max == (j - r) as i64 {
				piv_r = j;
			}
		}
 	}println!("OK");
	 for i in 0..(ub - lb + 1) as usize {
		if i + piv_p < q - p + 1 && (piv_r as i64) - (i as i64) >= 0 {
		grid[i + piv_p][piv_r - i] = '*';
		}
	}	
	for i in 0..(q - p + 1) {
		for j in &grid[i] {
			print!("{}",j);
		}
		println!("");
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
