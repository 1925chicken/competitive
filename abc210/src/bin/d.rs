use proconio::input;
#[allow(dead_code)]
fn solve(read:&mut snio::Reader<std::io::StdinLock<'_>>) {
    let h = read.usize();
    let w = read.usize();
    let c = read.i64();
    let mut a:Vec<Vec<_>> = vec![vec![0;w];h];
    let mut cc = vec![(0,0,0);h * w];
    for i in 0..h {
        for j in 0..w {
            a[i][j] = read.i64();
            cc[(i * w + j)] = (a[i][j],i,j);
        }
    }
    cc.sort();
    let mut ans = 1e18 as i64;
    ans = std::cmp::min(ans,cc[0].0 + cc[1].0 + ((cc[0].1 as i64 - cc[1].1 as i64).abs() + (cc[0].2 as i64 - cc[1].2 as i64).abs()) * c);
    for i in 0..h {
        for j in 0..w {
            for k in -11..=11{
                for l in -11..=11{
                let t = i as i64 + k;
                let u = j as i64 + l;
                if t < 0 || t >= h as i64 || u < 0 || u >= w as i64 || (k == 0 && l == 0) {
                    continue;
                }
                ans = std::cmp::min(a[i][j] + a[t as usize][u as usize] + ((k).abs() + (l).abs()) * c,ans);
            }
        }
    }
    }
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
fn bfs() {

}
fn bfs2() {

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