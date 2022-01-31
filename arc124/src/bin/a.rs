//use proconio::input;
#[allow(dead_code)]
fn solve(read:&mut snio::Reader<std::io::StdinLock<'_>>) {
    let n = read.usize();
    let m = read.usize();
    let mut right_sum = vec![0;n + 1];
    let mut left_sum = vec![0;n + 1];
    let mut st = std::collections::HashSet::new();
    let mut sum = vec![0;n + 1];
    for _ in 0..m {
        let c = read.char();
        let k = read.usize();
        st.insert(k - 1);
        if c == 'L' {
            left_sum[k - 1] += 1;
        }else {
            right_sum[k - 1] += 1;
        }
    }
    for i in 0..n {
        left_sum[n - i - 1] += left_sum[n - i];
        right_sum[i + 1] += right_sum[i];
    }
    //println!("{:?}\n{:?}",left_sum,right_sum);
    let mut ans = 1;
    for i in 0..n {
        if !st.contains(&i){
            ans *= (m as i64 - left_sum[i + 1] - right_sum[i]);
            ans %= 998244353;
        }

    }
   // println!("{:?}",sum);
    println!("{}",ans);
}
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
    }
}