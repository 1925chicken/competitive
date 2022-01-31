#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
	let n = read.usize();
	let mut g = vec![Vec::new();n];
	let mut time = vec![0;n];
	for i in 0..n {
		time[i] = read.i64();
		let k = read.usize();
		for _ in 0..k {
			g[i].push(read.usize() - 1);
		}
	}
	let mut st = std::collections::HashSet::new();
	let mut que = std::collections::VecDeque::new();
	let mut cost = vec![1e9 as i64;n];
	cost[n - 1] =0;
	que.push_front(n - 1);
	while let Some(now) = que.pop_front() {
		for from in &g[now] {
			if cost[*from] > 1 + cost[now] {
				st.insert(*from);
				cost[*from] = cost[now] + 1;
				que.push_back(*from);
			}
		}
	}
	st.insert(n - 1);
	let mut ans = 0;
	for val in st {
		ans += time[val];
	}
	println!("{}",ans);
	/*let mut dp = vec![1e18 as i64;n];
	let ans = dfs(&g,&mut dp,&time,n - 1);
	//println!("{:?}",dp);
	println!("{}",ans);*/
}
fn dfs(g:&Vec<Vec<usize>>,dp:&mut Vec<i64>,time:&Vec<i64>, now:usize) -> i64 {
	if dp[now] != 1e18 as i64 {
		return dp[now];
	}
	if g[now].len() == 0 {
		dp[now] = time[now];
		return dp[now];
	}
	let mut ret = 0;
	for from in &g[now] {
		ret = std::cmp::min(ret + dfs(g,dp,time,*from),dp[now]);
	}
	ret = std::cmp::min(dp[now],ret + time[now]);
	dp[now] = ret;
	ret
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
