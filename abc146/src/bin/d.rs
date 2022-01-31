#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
	let n = read.usize();
	let mut g = vec![Vec::new();n];
	let mut edge_index = std::collections::HashMap::new();
	for i in 0..n - 1 {
		let a = read.usize() - 1;
		let b = read.usize() - 1;
		g[a].push(b);
		g[b].push(a);
		edge_index.entry((a,b)).or_insert(i);
	}
	let mut ans = std::collections::BTreeMap::new();
	let mut que = std::collections::VecDeque::new();
	que.push_back((0,10000000000));
	let mut cols = 0;
	while let Some(now) = que.pop_front() {
		let mut cnt = 1;
		for to in &g[now.0] {
			if now.1 == *to {continue;}
			let k = ans.get(&edge_index.get(&(std::cmp::min(now.0,*to),std::cmp::max(now.0,*to))).unwrap());
			let l = edge_index.get(&(std::cmp::min(now.0,*to),std::cmp::max(now.0,*to))).unwrap();
				if k != Some(&cnt) {
					ans.entry(l).or_insert(cnt);
				}else {
					cnt += 1;
					ans.entry(l).or_insert(cnt);
				}
				que.push_back((*to,now.0));
				cols = std::cmp::max(cols,cnt);
		}
	}
	println!("{}",cols);
	for (_,val) in ans {
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
