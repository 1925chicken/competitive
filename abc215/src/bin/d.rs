#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
    let n = read.usize();
    let m = read.usize();
    let a:Vec<_> = (0..n).map(|_| read.usize()).collect();
    let mut facts = vec![0;1e5 as usize + 1];
    let min_factors = sieve_of_Eratosthenes(1e5 as usize);
    let mut st = std::collections::HashSet::new();
    for i in 0..n {
        let mut t = a[i];
        while t != 1 {
            st.insert(min_factors[t]);
            t /= min_factors[t] as usize;
        }
    }
    for val in st {
        let mut cnt = 1;
        while cnt * val <= m as i64 {
            facts[cnt as usize * val as usize] += 1;
            cnt += 1;
        }
    }
   // println!("{:?}",facts);
    let mut cnt = 0;
    let mut ans = Vec::new();
    for i in 1..=m {
        if facts[i] == 0 {
            cnt += 1;
            ans.push(i);
        }
    }
    println!("{}",cnt);
    for i in 0..ans.len() {
        println!("{}",ans[i]);
    }
}
fn sieve_of_Eratosthenes(n:usize) -> Vec<i64>{//0,1以外で,その数が素数ならmin_factors[i] == i
    let mut min_factors:Vec<i64> = (0..(n + 1)).map(|i| i as i64).collect();
    min_factors[0] = 1;
    min_factors[1] = 1;
    let mut i = 2;
    while i * i <= n as i64 {
        if min_factors[i as usize] == i {
            let mut j = 2;
            while j * i as usize <= n {
                if min_factors[i as usize * j] > i {
                    min_factors[i as usize * j] = i;
                }
                j += 1;
            }
        }
        i += 1;
    }
    min_factors
}
#[allow(dead_code)]
fn factrise(min_factors:&Vec<i64>,x:i64) -> Vec<(i64,i64)>{
    let mut tmp = x;
    let mut rec = Vec::new();
    let mut mp = std::collections::HashMap::new();
    while tmp != 1 {
        *mp.entry(min_factors[tmp as usize]).or_insert(0) += 1;
        tmp /= min_factors[tmp as usize];
    }
    for (factor,ex) in mp {
        rec.push((factor,ex));
    }
    rec.sort();
    rec
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
