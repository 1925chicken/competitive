#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
    let (h, a, b) = (read.i64(), read.i64(), read.i64());
    if a >= h {
        println!("YES");
        println!("{}", 1);
    } else if a < b {
        println!("NO");
    } else {
        println!("YES");
        println!("{}", binary_search(h, a, b));
    }
}
fn binary_search(h: i64, a: i64, b: i64) -> i64 {
    let (mut ub, mut lb) = ((1e9 as i64 + 5), 1);
    while ub - lb > 1 {
        let mid = (lb + ub) / 2;
        if mid * a - (mid - 1) * b >= h {
            ub = mid;
        } else {
            lb = mid;
        }
    }
    ub
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
            self.buf
                .pop_front()
                .unwrap_or_else(|| panic!("input ended"))
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
            self.string()
                .parse::<T>()
                .expect("Failed to parse the input.")
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

const INF: i64 = 1i64 << 60;
