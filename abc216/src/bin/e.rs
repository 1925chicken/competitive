#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
    let n = read.usize();
    let mut sum = 0;
    let mut k = read.i64() as i128;
    let a:Vec<_> = (0..n).map(|_| read.i64() as i128).collect();
    let mut que = std::collections::BinaryHeap::new();
    for i in 0..n {
        que.push(a[i]);
        sum += a[i];
    }
    let mut ans = 0;
    let mut flag = false;
    while k != 0 && !que.len() != 0 {
        let mut t = que.pop().unwrap();
       // println!("{:?} {}",que,ans);
        if que.len() == 0 {
            if k > t {
                ans += (t + 1) * t / 2;
                flag = true;
                break;
            }else {
                ans += (t + 1) * t / 2 - (t - k + 1) * (t - k) / 2;
                flag = true;
                break;
            }
        }else {
            let tt = que.pop().unwrap();
            que.push(tt);
            let mut dif = t - tt + 1;
            dif = std::cmp::min(k,dif);
            ans += (t + 1) * t / 2 - (t - dif) * (t - dif + 1) / 2;
            sum -= dif;
            k -= dif;
            t -= dif;
        }
        if sum <= k {
            que.push(t);
            break;
        }
        if t != 0 {
            if k == 0 {
            flag = true;
        }
            que.push(t);
        }
    }
    while let Some(val) = que.pop() {
        if flag {
            break;
        }
        ans += (val + 1) * val / 2;   
    }
   // println!("{:?}",que);
    println!("{}",ans);
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
