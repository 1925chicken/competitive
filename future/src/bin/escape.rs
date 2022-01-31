#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
    let n = read.usize();
    let m = read.usize();
    let l = read.i64();
    let s:Vec<_> = read.chars();
    let mut g = vec![Vec::new();n];
    let mut mp = std::collections::HashMap::new();
    for _ in 0..m {
        let (a,b,c) = (read.usize() - 1,read.usize() - 1,read.i64());
        g[a].push(b);
        g[b].push(a);
        mp.insert((std::cmp::min(a,b),std::cmp::max(a,b)),c);
    }
    let mut cost = bfs(&s,&g,&mp,l);
   /* for i in 1..n {
        if cost[i] != -1 {
        cost[i] = if bfs2(&s,&g,&mp,l,i,cost[i]) {cost[i]} else{-1};
        }
    }*/

    let mut cnt = 0;
    for i in 0..n {
        if cost[i] * 2 >= l {cnt += 1;}
    }
    println!("{}",cnt);
}
fn bfs(s:&Vec<char>,g:&Vec<Vec<usize>>,mp:&std::collections::HashMap<(usize,usize),i64>,l:i64) -> Vec<i64>{
    let mut cost = vec![-1;s.len()];
    cost[0] = l;
    let mut que = std::collections::BinaryHeap::new();
    que.push((l,0));
    while let Some(now) = que.pop() {
        let (now_cost,now_pos) = now;
        for to in &g[now_pos] {
            let pair = (std::cmp::min(now_pos,*to),std::cmp::max(now_pos,*to));
            let to_cost = *mp.get(&pair).unwrap();
            if to_cost > now_cost {
                continue;
            }
            else if to_cost == now_cost && s[*to] == '0' {continue;}
            else if to_cost <= now_cost && s[*to] == '1' {
                que.push((l,*to));
                cost[*to] = l;
            }else {
                if now_cost - to_cost <= cost[*to] {continue;}
                que.push((now_cost - to_cost,*to));
                cost[*to] = now_cost - to_cost;
            }
        }
    }
    return cost
}

fn bfs2(s:&Vec<char>,g:&Vec<Vec<usize>>,mp:&std::collections::HashMap<(usize,usize),i64>,l:i64,start:usize,now_c:i64) -> bool{
    let mut cost = vec![-1;s.len()];
    cost[start] = now_c;
    let mut que = std::collections::VecDeque::new();
    que.push_back((now_c,start,0));
    while let Some(now) = que.pop_front() {
        let (now_cost,now_pos,now_from) = (now.0,now.1,now.2);
        for to in &g[now_pos] {
            let pair = (std::cmp::min(now_pos,*to),std::cmp::max(now_pos,*to));
            let to_cost = *mp.get(&pair).unwrap();
            if now_from == *to {
                continue;
            }
            if to_cost > now_cost {
                continue;
            }
            else if to_cost == now_cost && s[*to] == '0' {continue;}
            else if to_cost <= now_cost && s[*to] == '1' {
                que.push_front((0,*to,now_pos));
                cost[*to] = l;
            }else {
                if now_cost - to_cost <= cost[*to] {continue;}
                que.push_back((now_cost - to_cost,*to,now_pos));
                cost[*to] = now_cost - to_cost;
            }
        }
    }
    if cost[0] != -1 {
        return true;
    }
    false
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
