#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
    let mut pq = std::collections::BinaryHeap::new();
    let mut que = std::collections::VecDeque::new();
    let mut qu = read.i64();
    let mut cnt = 0;
    while cnt < qu {
        let mut q = read.i64();
        if q == 3 {
            cnt += 1;
            break;
        }else if q == 2 {
            println!("{}",que.pop_front().unwrap());
        }else {
            let t = read.i64();
            que.push_back(t);
        }
        cnt += 1;
    }
    for val in que {
        pq.push(std::cmp::Reverse(val));
    }
    let mut que = std::collections::VecDeque::new();
    for i in 0..qu - cnt {
        let q = read.i64();
        if q == 1 {
            let t = read.i64();
            que.push_back(t);
        }else if q == 2 {
            if pq.len() == 0 {
                println!("{}",que.pop_front().unwrap());
            }else {
                println!("{}",pq.pop().unwrap().0);
            }
        }else {
            for val in que {
                pq.push(std::cmp::Reverse(val));
            }
            que = std::collections::VecDeque::<i64>::new();
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
