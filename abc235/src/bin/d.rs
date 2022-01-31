#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
	let mut a = read.i64();
	let mut n = read.i64();
	//let mut cnt = 0;
	let ff = n.to_string().len();
	let mut mp = std::collections::HashMap::new();
	let mut que = std::collections::VecDeque::new();
	que.push_front((1,0));
	mp.insert(1,0);
	while let Some((mut now,cnt)) = que.pop_front() {//println!("{}",now);
		if mp.contains_key(&now) && *mp.get(&now).unwrap() < cnt {continue;}
		if !mp.contains_key(&(now * a)) || (mp.contains_key(&(now * a)) && *mp.get(&(now * a)).unwrap() > cnt + 1) {
			if now.to_string().len() > ff {continue;}
			mp.insert((now * a),cnt + 1);
			que.push_back((now * a,cnt + 1));
		}
		if now >= 10 && now % 10 != 0 {
			let s:Vec<_> = now.to_string().chars().collect();
			let mut f = String::new();
			for i in 0..s.len() {
				f.push(s[(s.len() - 1 + i) % s.len()]);
			}
			now = f.parse().unwrap();
			if !mp.contains_key(&now) || (mp.contains_key(&now) && *mp.get(&(now)).unwrap() > cnt + 1) {
				mp.insert((now),cnt + 1);
				que.push_back((now,cnt + 1));
			}
		}
	}
	//println!("{:?}",mp);
	if !mp.contains_key(&n) {
		println!("{}",-1);
	}else {
		println!("{}",*mp.get(&n).unwrap());
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
