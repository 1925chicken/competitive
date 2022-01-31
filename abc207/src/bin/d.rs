//use proconio::input;
#[allow(dead_code)]
fn solve(read:&mut snio::Reader<std::io::StdinLock<'_>>) {
    let n = read.usize();
    let mut ab:Vec<_> = (0..n).map(|_| (read.i64(),read.i64())).collect();
    let mut cd:Vec<_> = (0..n).map(|_| (read.i64(),read.i64())).collect();
    let mut arg = geometry::ArgumentSort::new();
    let ab = arg.sorting(ab,false);
    let cd = arg.sorting(cd,false);
    println!("{:?} {:?}",ab,cd);
    if n == 1 {
        println!("Yes");
        return;
    }
    
    for i in 0..n {
        let mut flg = true;
        for j in 0..n {
           let a = ab[(i * 3 + j) % n];
           let b = ab[(i * 3 + j + 1) % n];
           let c = cd[j];
           let d = cd[(j + 1) % n];
           if (a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1) != (c.0 - d.0) * (c.0 - d.0) + (c.1 - d.1) * (c.1 - d.1) {
               flg = false;
           } 
        }
        if flg {
            println!("Yes");
            return;
        }
    }
    println!("No");
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
//Original -> https://github.com/DivineJK/my_python_libraries/tree/master/Geometry
pub mod geometry {
    pub struct ArgumentSort {
        start_vec:(i64,i64),
        zero_arg:(i64,i64),
        left_include:u8,
    }
    impl ArgumentSort {
        pub fn new() -> Self {
            ArgumentSort {
                start_vec:(1,0),
                zero_arg:(1,0),
                left_include:0,
            }
        }
        pub fn set_start_vec(&mut self, sv:(i64,i64)) {
            self.start_vec = match sv {
                (0,0) => (1,0),
                _ => sv,
            };
        }
        pub fn set_zero_arg(&mut self, za:(i64,i64)) {
            self.zero_arg = match za {
                (0,0) => (1,0),
                _ => za,
            };
        }
        pub fn set_left_include(&mut self, li:u8) {
            self.left_include = li;
        }
        fn inner_product(& self, a:(i64,i64),b:(i64,i64)) -> i64 {
            a.0 * b.0 + a.1 * b.1
        }
        fn outer_product(& self, a:(i64,i64),b:(i64,i64)) -> i64 {
            a.0 * b.1 - a.1 * b.0
        }
        fn chk_zone(& self,mut a:(i64,i64)) -> u8 {
            if a == (0,0) {
                a = self.zero_arg;
            }
            let ip = self.inner_product(self.start_vec,a);
            let op = self.outer_product(self.start_vec,a);
            let t = {if ip > 0 && op == 0 {
                0
            }else if ip > 0 && op > 0 {
                1
            }else if ip == 0 && op > 0 {
                2
            }else if ip < 0 && op > 0 {
                3
            }else if ip < 0 && op == 0 {
                4
            }else if ip < 0 && op < 0 {
                5
            }else if ip == 0 && op < 0 {
                6
            }else {
                7
            }
        };
            t
        }
        fn compare_argument(&mut self,a:(i64,i64),b:(i64,i64)) -> bool {
            if self.chk_zone(a) != self.chk_zone(b) {
                return self.chk_zone(a) < self.chk_zone(b);
            }
            if self.outer_product(a,b) != 0 {
                return self.outer_product(a,b) > 0;
            }
            self.inner_product(a,a) <= self.inner_product(b,b)
        }
        pub fn sorting(&mut self,vecs:Vec<(i64,i64)>,rev:bool) -> Vec<(i64,i64)> {//同じx(y)座標なら原点から近い順に並ぶよ
            let n = vecs.len();
            if n == 1 {
                return vecs;
            }
            let (mut l ,mut r) = (Vec::new(),Vec::new());
            for i in 0..n/2 {
                l.push(vecs[i]);
            }
            for i in n/2..n {
                r.push(vecs[i]);
            }
            let left = self.sorting(l,rev);
            let right = self.sorting(r,!rev);
            let mut pnt_l = 0;
            let mut pnt_r = n as i64 - n as i64 / 2 - 1;
            let mut idx_r = vec![0;n];
            for i in 0..n {
                if pnt_l >= n / 2 {
                    idx_r[i] = pnt_r as usize + n / 2;
                    pnt_r -= 1;
                }else if pnt_r < 0 {
                    idx_r[i] = pnt_l;
                    pnt_l += 1;
                }else {
                    if self.compare_argument(left[pnt_l],right[pnt_r as usize]) {
                        if rev {
                            idx_r[i] = pnt_r as usize + n / 2;
                            pnt_r -= 1;
                        }else {
                            idx_r[i] = pnt_l;
                            pnt_l += 1;
                        }
                    }else {
                        if rev {
                            idx_r[i] = pnt_l;
                            pnt_l += 1;
                        }else {
                            idx_r[i] = pnt_r as usize + n / 2;
                            pnt_r -= 1;
                        }
                    }
                }
            }
            let mut res = vec![(0,0);n];
            for i in 0..n {
                if idx_r[i] >= n / 2 {
                    res[i] = right[idx_r[i] - n / 2];
                }else {
                    res[i] = left[idx_r[i]];
                }
            }
            res
        }
    }
}
