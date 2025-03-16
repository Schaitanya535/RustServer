pub const MOD: i64 = 1000_000_000 + 7;

pub fn combinations(n: i64, r: i64) -> i64 {
    (1..r.min(n - r)).fold(1, |acc, val| (acc * (n - val) / val))
}

pub fn gcd(a: i32, b: i32) -> i32 {
    let reminder = a.max(b) % b.min(a);
    if reminder == 0 {
        return b.min(a);
    }
    return gcd(reminder, b.min(a));
}

pub fn pow_mod(base: i32, exp: i32) -> i32 {
    let mut base = base as i64;
    let mut exp = exp;
    let mut result = 1;
    while exp > 0 {
        if exp & 1 > 0 {
            result = (result * base) % MOD;
        }
        base = (base * base) % MOD;
        exp = exp >> 1;
    }
    (result % MOD) as i32
}

pub fn factorial(n: i32) -> i32 {
    (1..=(n as i64)).fold(1 as i64, |acc: i64, i| (acc * i) % MOD) as i32
}

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, u: usize) -> usize {
        if self.parent[u] == u {
            return u;
        }
        self.parent[u] = self.find(self.parent[u]);
        self.parent[u]
    }

    fn union(&mut self, mut u: usize, mut v: usize) -> bool {
        u = self.find(u);
        v = self.find(v);
        if u == v {
            return false;
        }
        if self.rank[u] > self.rank[v] {
            std::mem::swap(&mut u, &mut v);
        }
        self.parent[u] = v;
        if self.rank[u] == self.rank[v] {
            self.rank[v] += 1;
        }
        true
    }
}

pub fn decode(string: &str) -> Result<(String, i32), Box<dyn std::error::Error>> {
    let (header, content) = string.split_once("\r\n\r\n").ok_or("err")?;
    let content_len: i32 = header.split_once(":").ok_or("err")?.1.parse()?;

    return Ok((content.to_string(), content_len));
}
