#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
	let n = read.usize();
	let q = read.usize();
	let a:Vec<_> = (0..n).map(|_| read.i64()).collect();//座圧
	let mut st = std::collections::BTreeSet::new();
	let mut mp = std::collections::HashMap::new();
	for i in 0..n {
		st.insert(a[i]);
	}
	let mut cnt = 0;
	for i in &st {
		mp.insert(*i,cnt);
		cnt += 1;
	}
	let mut rec = vec![0;cnt];
	let mut mp2 = std::collections::HashMap::new();
	for i in 0..n {
		rec[*mp.get(&a[i]).unwrap()] += 1;
		mp2.insert((*mp.get(&a[i]).unwrap(),rec[*mp.get(&a[i]).unwrap()]),i);
	}
	let mut ans = Vec::new();
	for i in 0..q {
		let x = read.i64();
		let k = read.i64();
		if !st.contains(&x) || !mp2.contains_key(&(*mp.get(&x).unwrap(),k)) {
			ans.push(-1);
		}else {
			ans.push(*mp2.get(&(*mp.get(&x).unwrap(),k)).unwrap() as i64 + 1);
		}
	}
	for i in ans {
		println!("{}",i);
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
