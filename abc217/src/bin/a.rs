#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
    let n = read.usize();
    for _ in 0..n {
        let a = read.i128();
        let mut f = factrise_vec(a);
        f.sort();
        print!("{} ",f.len());
        for i in 0..f.len() {
            if i == f.len() - 1 {
                print!("{}",f[i].0);
            }else {
            print!("{} ",f[i].0);
            }
        }
        println!("");
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
        u8,u16,u32,u64,usize,i128,
        i8,i16,i32,i64,isize,
        f32,f64,
    }
}

const INF:i64 = 1i64 << 60;

fn factrise_vec(n: i128) -> Vec<(i128, i32)> {
    let mut rec = Vec::new();
    let mut now = n;
    loop {
        let mut div = now;
        if div != 1 {
        while !miller_rabin(div, 100){
            div = search_factor(div);
            println!("{}",div);
        }
        let mut cnt = 0;
        while now % div == 0 {
            now /= div;
            cnt += 1;
        }
        rec.push((div.clone(), cnt));
    }
        if now == 1 {
            break;
        }
    }
    rec
}

fn factrise_map(n: i128) -> std::collections::HashMap<i128, i32> {
    let mut mp = std::collections::HashMap::<i128, i32>::new();
    let mut now = n;
    loop {
        let mut div = n;
        while !miller_rabin(div, 100) {
            div = search_factor(div);
        }
        let mut cnt = 0;
        while now % div == 0 {
            now /= div;
            cnt += 1;
        }
        *mp.entry(div).or_insert(0) += cnt;
        if now == 1 {
            break;
        }
    }
    mp
}

fn call(n: i128, modulo: i128, c: i128) -> i128 {
    (n * n + c) % modulo
}

fn search_factor(n: i128) -> i128 {
    let mut c = 1;
    let mut y = 0;
    let (mut x, mut r, mut q, mut g, mut ys, mut m) = (1, 1, 1, 1, 1, 1);
    loop {
        let mut x = y;
        let mut cnt = 0;
        while cnt < r {
            y = call(y, n, c);
            cnt += 1;
        }
        let mut k = 0;
        loop {
            ys = y;
            let mut cnt = 0;
            let mut lim = std::cmp::min(m, r - k);
            for _ in 0..lim {
                y = call(y, n, c);
                q *= (x - y).abs();
                q %= n;
                cnt += 1;
            }
            g = gcd(q, n);
            k = k + m;
            if k >= r || g > 1 {
                break;
            }
        }
        r = 2 * r;
        if g > 1 {
            break;
        }
    }
    c += 11;
    if g == n {
        loop {
            let mut cnt = 0;
            ys = call(ys, n, c);
            g = gcd((x - ys).abs(), n);
            if g > 1 {
                break;
            }
            cnt += 1;
            if cnt == 10000 {
                cnt = 0;
                c += 11;
            }
        }
    }
    if g == n {
        return n;
    } else {
        g
    }
}

static mut XORSHIFT1: i128 = 123456789;
static mut XORSHIFT2: i128 = 362436069;
static mut XORSHIFT3: i128 = 521288629;
static mut XORSHIFT4: i128 = 88675123;
fn xorshift() -> i128 {
    unsafe {
        let mut t = 0;
        t = XORSHIFT1 ^ (XORSHIFT1 << 11i128);
        XORSHIFT1 = XORSHIFT2;
        XORSHIFT2 = XORSHIFT3;
        XORSHIFT3 = XORSHIFT4;
        XORSHIFT4 = (XORSHIFT4 ^ (XORSHIFT4 >> 19i128)) ^ (t ^ (t >> 8i128));
        t
    }
}
fn witness(n: i128, a: i128) -> bool {
    let mut t = 0;
    let mut u = n - 1;
    while u & 1i128 == 0 {
        t += 1;
        u >>= 1;
    }
    let mut x = modpow(a, u, n);

    for _ in 0..t {
        let b = x * x % n;
        if b == 1 {
            if b != 1 && b != n - 1 {
                return true;
            } else {
                return false;
            }
        }
        x = b;
    }
    true
}
fn miller_rabin(n: i128, times: i64) -> bool {
    if n == 1 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }

    for _ in 0..times {
        //let mut a = rand::thread_rng().gen_range(1,n);
        let mut a = xorshift() % n;
        while a == 0 {
            a = xorshift() % n;
        }
        if witness(n, a) {
            return false;
        }
    }
    true
}
fn modpow(_x: i128, _n: i128, modulo: i128) -> i128 {
    if _n == 0 {
        return 1;
    }
    if _n % 2 == 0 {
        let t = modpow(_x, _n / 2, modulo);
        return t * t % modulo;
    }
    _x * modpow(_x, _n - 1, modulo)
}
#[allow(dead_code)]
fn gcd<T>(a: T, b: T) -> T
where
    T: std::cmp::Ord
        + Copy
        + std::ops::Rem<Output = T>
        + Zero
        + std::marker::Sized
        + std::ops::Mul<Output = T>
        + std::ops::MulAssign
        + MinusOne
        + std::ops::Div<Output = T>
        + std::ops::DivAssign,
{
    let (mut x, mut y) = (a, b);
    if x < T::zero() {
        x *= T::minusone();
    }
    if y < T::zero() {
        y *= T::minusone();
    }
    if x < y {
        std::mem::swap(&mut x, &mut y);
    }
    let mut r = y;
    while !T::is_zero(&r) {
        r = x % y;
        x = y;
        y = r;
    }
    x
}
#[allow(dead_code)]
fn lcm<T>(x: T, y: T) -> T
where
    T: std::cmp::Ord
        + Copy
        + std::ops::Rem<Output = T>
        + Zero
        + std::marker::Sized
        + std::ops::Mul<Output = T>
        + std::ops::MulAssign
        + MinusOne
        + std::ops::Div<Output = T>
        + std::ops::DivAssign,
{
    x / gcd(x, y) * y
}
trait Zero: std::ops::Rem<Output = Self> + std::marker::Sized {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}
trait MinusOne: std::ops::Mul<Output = Self> + std::marker::Sized {
    fn minusone() -> Self;
}

macro_rules! impl_zero {
    ($($t:ty),*)=> {
        $(
            impl Zero for $t {
                fn zero() -> Self{
                    0
                }
                fn is_zero(&self) -> bool {
                    *self == 0
                }
            }
        )*
    };
}

macro_rules! impl_minusone {
    ($($t:ty),*)=> {
        $(
        impl MinusOne for $t {
            fn minusone() -> Self {
                -1
            }
        }
        )*
    };
}
impl_zero! {u64,usize,i64,i32,u32,isize,i128}
impl_minusone! {i64,i32,isize,i128}