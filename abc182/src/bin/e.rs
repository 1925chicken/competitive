//use proconio::input;
#[allow(dead_code)]
fn solve(read:&mut snio::Reader<std::io::StdinLock<'_>>) {
    let h:usize = read.usize();
    let w:usize = read.usize();
    let n = read.i32();
    let m = read.i32();
    let mut flags = vec![vec![0 as i8;w];h];
    let mut st = std::collections::HashSet::new();
    for _ in 0..n {
        let a = read.usize();
        let b = read.usize();
        flags[a - 1][b - 1] = 1;
        st.insert((a - 1,b - 1));
    }
    for _ in 0..m {
        let a = read.usize();
        let b = read.usize();
        flags[a - 1][b - 1] = -1;
    }
    let mut left = vec![vec![0;w];h];
    let mut right = vec![vec![0;w];h];
    let mut top = vec![vec![0;w];h];
    let mut bottom = vec![vec![0;w];h];
    for i in 0..h {
        let mut cnt = 0;
        for j in 0..w {
            if flags[i][j] == -1 {
                cnt = 0;
            }else if flags[i][j] == 1{
                cnt = 1;
            }
            left[i][j] = cnt;
        }
    }
    for i in 0..h {
        let mut cnt = 0;
        for j in (0..w).rev() {
            if flags[i][j] == -1 {
                cnt = 0;
            }else if flags[i][j] == 1 {
                cnt = 1;
            }
            right[i][j] = cnt;
        }
    }
    for j in 0..w {
        let mut cnt = 0;
        for i in 0..h {
            if flags[i][j] == -1 {
                cnt = 0;
            }else if flags[i][j] == 1 {
                cnt = 1;
            }
            top[i][j] = cnt;
        }
    }
    for j in 0..w {
        let mut cnt = 0;
        for i in (0..h).rev() {
            if flags[i][j] == -1 {
                cnt = 0;
            }else if flags[i][j] == 1 {
                cnt = 1;
            }
            bottom[i][j] = cnt;
        }
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if left[i][j] == 1 || right[i][j] == 1 || top[i][j] == 1 || bottom[i][j] == 1 {
                ans += 1;
            }
        }
    }
    //println!("{:?}",flags);
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