//use proconio::input;
#[allow(dead_code)]
fn solve(read:&mut snio::Reader<std::io::StdinLock<'_>>) {
    let n = read.usize();
    let q = read.usize();
    let mut a:Vec<_> = (0..n).map(|_| read.i64()).collect();
    a.push(0);
    a.sort();
    let mut sum = vec![0;n + 1];
    let mut k:Vec<_> = (0..q).map(|_| read.i64()).collect();
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i + 1] - a[i] - 1;
    }
    //println!("{:?}",sum);
    for i in 0..q {
        let t = lower_bound(&sum,k[i]) - 1;
        //println!("{}",t);
        println!("{}",a[t] + k[i] - sum[t]);
    }
}
use std::iter::Iterator;
#[allow(dead_code)]
fn upper_bound<T>(f:&Vec<T>,criteria:T) -> usize //criteriaより大きい要素が最初にどこに現れるかを返す
where
    T:std::cmp::Ord,
{
    let las = f.len();
    let mut fir = 0;
    let mut length = las;
    while length != 0{
        let half = length / 2;
        let mut mid = fir;
        mid += half;
        if criteria >= f[mid] {
            length -= half + 1;
            mid += 1;
            fir = mid;
        }else{
            length = half;
        }
    }
    fir
}
#[allow(dead_code)]
fn lower_bound<T>(f:&Vec<T> , criteria:T) -> usize //criteria以上の要素が最初にどこに現れるかを返す
where
    T:std::cmp::Ord,
{
    let las = f.len();
    let mut fir = 0;
    let mut length = las;
    while length != 0 {
        let half = length / 2;
        let mut mid = fir;
        mid += half;
        if f[mid] < criteria {
            length -= half + 1;
            mid += 1;
            fir = mid;
        }else {
            length = half;
        }
    }
    fir
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