#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
    let m = read.usize();
    let mut g = vec![Vec::new(); 9];
    for _ in 0..m {
        let a = read.usize() - 1;
        let b = read.usize() - 1;
        g[a].push(b);
        g[b].push(a);
    }
    let mut pieces = [8; 9];
    for i in 0..8 {
        let p = read.usize() - 1;
        pieces[p] = i;
    }
    //println!("{:?}",pieces);
    println!("{}", bfs(&g, &mut pieces));
}

fn bfs(g: &Vec<Vec<usize>>, pieces: &mut [usize; 9]) -> i64 {
    let mut que = std::collections::VecDeque::new();
    que.push_back(pieces.clone());
	let mut mp = std::collections::HashMap::new();
	mp.entry(pieces.clone()).or_insert(0);
	while let Some(mut state) = que.pop_front() {
		let mut v = 0;
		for i in 0..9 {
			if state[i] == 8 {
				v = i;
			}
		}
		let cost = *mp.get(&mut state).unwrap();
		for to in &g[v] {
			state.swap(*to,v);
			if mp.contains_key(&state) {
				state.swap(*to,v);
				continue;
			}
			mp.entry(state).or_insert(cost + 1);
			que.push_back(state);
			state.swap(*to,v);
		}
	}
	//println!("{:?}",mp);
	if mp.get(&[0,1,2,3,4,5,6,7,8]) == None {
		return -1;
	}
	*mp.get(&[0,1,2,3,4,5,6,7,8]).unwrap()
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
