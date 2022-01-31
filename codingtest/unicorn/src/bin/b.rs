#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
	let n = read.usize();
    let m = read.usize();
    let mut g = vec![Vec::new(); n];
    for i in 0..n {
        g[i] = read.chars();
    }
    let mut que = std::collections::VecDeque::new();
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if g[i][j] == 'S' {
                start = (i, j);
            }
            if g[i][j] == 'G' {
                goal = (i, j);
            }
        }
    }
    que.push_front((start,0,0));
    let mut cost = vec![vec![INF; m]; n];
    cost[start.0][start.1] = 0;
	while let Some(now) = que.pop_front() {
		let cost_now = now.2;
		let (ny,nx) = ((now.0).0 as i64 ,(now.0).1 as i64);
		let broken_wall = now.1;
		for (y,x) in [(0,1),(0,-1),(1,0),(-1,0)].iter() {
			if 0 <= ny + y && ny + y < n as i64 && 0 <= nx + x && nx + x < m as i64 {
				let (to_y,to_x) = ((ny + y) as usize,(nx + x) as usize);
				if g[to_y][to_x] == '#' {
					if cost[to_y][to_x] <= 1 + broken_wall + cost_now {continue;}
					cost[to_y][to_x] = 1 + broken_wall + cost_now;
					que.push_back(((to_y,to_x),broken_wall + 1,cost[to_y][to_x]));
				} else {
					if cost[to_y][to_x] <= 1 + cost_now {continue;}
					cost[to_y][to_x] = 1 + cost_now;
					que.push_front(((to_y,to_x),broken_wall,cost[to_y][to_x]));
				}
			}
		}
	}
	println!("{}",cost[goal.0][goal.1]);
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
