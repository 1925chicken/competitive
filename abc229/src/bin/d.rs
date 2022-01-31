#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
	let s:Vec<_> = read.chars();
	let mut k = read.i64();
	let mut sum = vec![0;s.len() + 1];
	for i in 0..s.len() {
		sum[i + 1] = sum[i] + if s[i] == '.' {0} else {1};
	}
	println!("{}",binary_search(&sum,k));
}

fn binary_search(sum:&Vec<i64>,k:i64) -> i64 {
	let (mut lb ,mut ub) = (0, 2e5 as i64);
	while ub - lb > 1 {
		let mid = (ub + lb) / 2;
		if judge(sum,k,mid) {
			lb = mid;
		}else {
			ub = mid;
		}
	}
	lb
}
fn judge(sum:&Vec<i64>,k:i64,target:i64) -> bool {
	let mut flag = false;
	let mut i = 1;
	while i + (target as usize) <= sum.len() {
		let t = sum[i + target as usize - 1] - sum[i];
		if t <= k && k +  >= target{
			flag = true;
		}
		i += 1;
	}
	flag
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
