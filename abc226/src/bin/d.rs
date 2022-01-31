#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
	let n = read.usize();
	let mut ab:Vec<_> = (0..n).map(|_| (read.i64(),read.i64())).collect();
	let mut st = std::collections::HashSet::new();
	for i in 0..n {
		for j in 0..n {
			if i == j {continue;}
			let aa = ab[i].0 - ab[j].0;
			let bb = ab[i].1 - ab[j].1;
			let gcd_ab = gcd(aa,bb);
			st.insert((aa/gcd_ab,bb/gcd_ab));
		}
	}
	println!("{}",st.len());
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
fn gcd<T>(a:T, b:T) -> T
where
	T:std::cmp::Ord + Copy + std::ops::Rem<Output=T> + Zero + std::marker::Sized + 
	std::ops::Mul<Output=T> + std::ops::MulAssign + MinusOne + std::ops::Div<Output=T> + std::ops::DivAssign,
{
	let (mut x,mut y) = (a,b);
	if x < T::zero() {
		x *= T::minusone();
	}
	if y < T::zero() {
		y *= T::minusone();
	}
	if x < y{
		std::mem::swap(&mut x,&mut y);
	}
	let mut r = y;
	while !T::is_zero(&r) {
		r = x % y;
		x = y;
		y = r;
	}
	x
}
#[allow(dead_code)]
fn lcm<T>(x:T,y:T) -> T 
where
	T:std::cmp::Ord + Copy + std::ops::Rem<Output=T> + Zero + std::marker::Sized + 
	std::ops::Mul<Output=T> + std::ops::MulAssign + MinusOne + std::ops::Div<Output=T> + std::ops::DivAssign,
{
	x / gcd(x,y) * y
}
trait Zero:std::ops::Rem<Output=Self> + std::marker::Sized {
	fn zero() -> Self;
	fn is_zero(&self) -> bool;
}
trait MinusOne:std::ops::Mul<Output=Self> + std::marker::Sized {
	fn minusone() -> Self;
}

macro_rules! impl_zero {
	($($t:ty),*)=> {
		$(
			impl Zero for $t {
				fn zero() -> Self{
					0
				}
				fn is_zero(&self) -> bool {
					*self == 0
				}
			}
		)*   
	};
}

macro_rules! impl_minusone {
	($($t:ty),*)=> {
		$(
		impl MinusOne for $t {
			fn minusone() -> Self {
				-1
			}
		}
		)*
	};
}
impl_zero! {u64,usize,i64,i32,u32,isize}
impl_minusone! {i64,i32,isize}


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
