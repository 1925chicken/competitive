#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
	let n = read.usize();
	let m = read.usize();
	let h:Vec<_> = (0..n).map(|_| read.i64() * (-1)).collect();
	let mut g:Vec<Vec<_>> = vec![Vec::new();n];
	for _ in 0..m {
		let mut a = read.usize() - 1;
		let mut b = read.usize() - 1;
		if h[a] > h[b] {
			g[a].push((b,2 * (h[a] - h[b])));
			g[b].push((a,(h[b] - h[a])));
		}else {
			std::mem::swap(&mut a,&mut b);
			g[a].push((b,2 * (h[a] - h[b])));
			g[b].push((a,(h[b] - h[a])));
		}
	}
	//println!("{:?}",g);
	let mut dist = vec![1e18 as i64;n];
	dijkstra(&g,&mut dist,0);
	println!("{}",dist.iter().min().unwrap() * (-1));
}
use std::collections::VecDeque;
use std::cmp::Reverse;
fn dijkstra(g:& Vec<Vec<(usize,i64)>>, dist:&mut Vec<i64>, start:usize) {
	let mut pq = VecDeque::new();
	pq.push_front((0,start));
	dist[start] = 0;
	while let Some(v) = pq.pop_front() {
		let (cost,now) = v;
		for (to,c) in &g[now] {
			if dist[now] < cost {
				continue;
			}
			if dist[*to] > *c + dist[now] {
				dist[*to] = dist[now] + *c;
				pq.push_back((dist[*to],*to));
			}
		}
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
