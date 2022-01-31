//use proconio::input;
#[allow(dead_code)]
fn solve(read:&mut snio::Reader<std::io::StdinLock<'_>>) {
    let n = read.usize();
    let m = read.usize();
    let l = read.usize();
    set_mod_int(1e9 as i64 + 7);
    let mut ans = mod_int::ModInt::new(0);
    let mut bi = binomial_coefficient::BinomialCoefficient::new(301,1e9 as i64 + 7);

}
use crate::mod_int::{set_mod_int,ModInt};
//from https://github.com/kenkoooo/competitive-programming-rs/blob/master/src/math/mod_int.rs
pub mod mod_int {
    use std::cell::RefCell;
    use std::ops::{Add,AddAssign,Div,DivAssign,Mul,MulAssign,Sub,SubAssign};
 
    type InternalNum = i64;
    thread_local!(
        static MOD: RefCell<InternalNum> = RefCell::new(0);//0という値でRefCellを生成
        );
 
    pub fn set_mod_int<T>(v:T)
    where
        InternalNum: From<T>,
    {
        MOD.with(|x| x.replace(InternalNum::from(v)));
    }
 
    fn modulo() -> InternalNum {
        MOD.with(|x| *x.borrow())
    }
    
    pub struct ModInt(InternalNum);//tuple構造体
    
    impl Clone for ModInt {
        fn clone(&self) -> Self {
            Self(self.0)
        }
    }
    impl Copy for ModInt {}
 
    impl ModInt {
        pub fn new<T>(v:T) -> Self
        where
            InternalNum: From<T>,
        {
            let mut v = InternalNum::from(v);
            let m = modulo();
            if v >= m {
                v %= m;
            }
            Self(v)
        }
 
        pub fn internal_pow(&self, mut e: InternalNum) -> Self {
            let mut result = 1;
            let mut cur = self.0;
            let modulo = modulo();
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                    result %= modulo;
                }
                e >>= 1;
                cur = (cur * cur) % modulo;
            }
            Self(result)
        }
 
        pub fn pow<T>(&self , e:T) -> Self
        where
            InternalNum: From<T>,
        {
            self.internal_pow(InternalNum::from(e))
        }
        
        pub fn value(&self) -> InternalNum {
            self.0
        }
    }
 
    impl From<ModInt> for InternalNum {
        fn from(m:ModInt) -> Self {
            m.value()
        }
    }
 
    impl<T> AddAssign<T> for ModInt
    where
        InternalNum: From<T>,
    {
        fn add_assign(&mut self,rhs:T) {
            let mut rhs = InternalNum::from(rhs);
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
 
            self.0 += rhs;
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }
 
    impl<T> Add<T> for ModInt
    where
        InternalNum: From<T>,
    {
        type Output = ModInt;
        fn add(self, rhs: T) -> Self::Output {
            let mut res = self;
            res += rhs;
            res
        }
    }
 
    impl<T> SubAssign<T> for ModInt
    where
        InternalNum: From<T>,
    {
        fn sub_assign(&mut self, rhs: T) {
            let mut rhs = InternalNum::from(rhs);
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            if rhs > 0 {
                self.0 += m - rhs;
            }
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }
    
    impl<T> Sub<T> for ModInt
    where
        InternalNum: From<T>,
    {
        type Output = Self;
        fn sub(self,rhs:T) -> Self::Output {
            let mut res = self;
            res -= rhs;
            res
        }
    }
 
    impl<T> MulAssign<T> for ModInt
    where
        InternalNum: From<T>,
    {
        fn mul_assign(&mut self, rhs: T) {
            let mut rhs = InternalNum::from(rhs);
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            self.0 *= rhs;
            self.0 %= m;
        }
    }
 
    impl<T> Mul<T> for ModInt
    where
        InternalNum: From<T>,
    {
        type Output = Self;
        fn mul(self, rhs:T) -> Self::Output {
            let mut res = self;
            res *= rhs;
            res
        }
    }
 
    impl<T> DivAssign<T> for ModInt
    where
        InternalNum: From<T>,
    {
        fn div_assign(&mut self, rhs:T){
            let mut rhs = InternalNum::from(rhs);
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            let inv = Self(rhs).internal_pow(m - 2);
            self.0 *= inv.value();
            self.0 %= m;
        }
    }
 
    impl<T> Div<T> for ModInt
    where
        InternalNum: From<T>,
    {
        type Output = Self;
        fn div(self, rhs: T) -> Self::Output {
            let mut res = self;
            res /= rhs;
            res
        }
    }
}
 
pub mod binomial_coefficient {
    use crate::mod_int::{set_mod_int,ModInt};
    pub struct BinomialCoefficient<ModInt> {
        fact:Vec<ModInt>,
        inversion:Vec<ModInt>
    }
    impl BinomialCoefficient<ModInt> {
       pub fn new(n:usize,_mod:i64) -> Self {
            set_mod_int(_mod);
            let a = ModInt::new(1);
            let mut _fact:Vec<ModInt> = vec![a;n + 1];
            let mut _inversion:Vec<ModInt> = vec![a;n + 1];
            for i in 1..=n{
                _fact[i] = _fact[i - 1] * i as i64;
            }
            _inversion[n] = _fact[n].internal_pow(_mod - 2);
            for i in (1..=n).rev(){
               _inversion[i - 1] = _inversion[i] * i as i64;
            }
        BinomialCoefficient {
            fact:_fact,
            inversion:_inversion
            }
        }
        pub fn combination(&mut self,n:i64,k:i64) -> ModInt{
            if k < 0 || k > n {
                return ModInt::new(0)    
            }
            self.fact[n as usize] * self.inversion[k as usize] * self.inversion[n as usize - k as usize]
 
        }
 
        pub fn permutation(&mut self,n:i64,k:i64) -> ModInt{
            if k < 0 || k > n{
                return ModInt::new(0)
            }
            self.combination(n,k) * self.fact[k as usize]
        }
    }
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