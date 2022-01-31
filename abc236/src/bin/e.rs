#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
	let n = read.usize();
	let a:Vec<_> = (0..n).map(|_| read.i64()).collect();
	let mut odd_id = Vec::new();
	let mut even_id = Vec::new();
	let mut even_sum = 0;
	let mut odd_sum = 0;
	for i in 0..n {
		if i % 2 == 0 {
			even_id.push(a[i]);
			even_sum += a[i];
		}else {
			odd_id.push(a[i]);
			odd_sum += a[i];
		}
	}
	even_id.sort_by_key(|&x| std::cmp::Reverse(x));
	odd_id.sort_by_key(|&x| std::cmp::Reverse(x));
	let mut ans1 = Total(0.0);
	let f = Total(binary_search(even_sum,&odd_id,(n as i64)/ 2));
	ans1 = std::cmp::max(ans1,f);
	let g = Total(binary_search(odd_sum,&even_id,(n as i64)/ 2));
	ans1 = std::cmp::max(ans1,g);
	println!("{}",(ans1.0));
}
fn binary_search(sum:i64,candidate:&Vec<i64>,length:i64) -> f64 {
	let (mut lb ,mut ub) = (0.0,1e15);
	let mut cnt = 0;
	while cnt < 60 {
		let mid = (lb + ub) / 2.0;
		if judge(sum,candidate,mid,length) {
			lb = mid;
		}else {
			ub = mid;
		}
		cnt += 1;
	}
	ub
}
fn judge(sum:i64,candidate:&Vec<i64>,cri:f64,length:i64)-> bool {
	let mut flag = false;
	let mut now = sum;
	for i in 0..candidate.len() {
		if (sum + candidate[i]) as f64 - cri * (length + 1) as f64 >= 0.0 {
			flag = true;
			break;
		}
	}
	flag
}// Partial orderなものをTotal orderにする
#[derive(PartialEq, PartialOrd)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
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
