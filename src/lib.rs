mod utils;

use std::collections::{HashMap, BTreeMap, VecDeque};

use js_sys::BigInt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn square(n: i32) -> BigInt {
    (n * n).into()
}

fn binary(n: i32) -> BigInt {
    let s: Vec<i32> = vec![1,1,1,1,0,0,0,0,1,1,0,1,0,0,1,0];
    (s[(n as usize) % s.len()]).into()
}

fn rueppel(n: i32) -> BigInt {
    if n <= 0 {
        0.into()
    } else {
        if 2_i32.pow(n.ilog2()) == n { 1.into() } else { 0.into() }
    }
}

fn rook(n: i32) -> BigInt {
    if n == 0 {
        0.into()
    } else if n < 0 {
        <i32 as Into<BigInt>>::into(1)-rook(-n)
    } else if n % 2 == 0 {
        rook(n / 2)
    } else {
        (((n-1)/2) % 2).into()
    }
}

fn knight(n: i32) -> BigInt {
    rook(n+1) - rook(n-1)
}

// start is the offset for the index to allow for storing data in an arbitrary
// place in 1-D space
#[derive(Debug)]
struct OffsetArray<T> {
    start: i32,
    array: VecDeque<T>
}


// need to start with a value since it will be off by one otherwise, or there
// needs to be an empty check or empty data type that gets promoted
impl<T> OffsetArray<T> {
    fn new(n: i32, v: T) -> Self {
        OffsetArray { start: n, array: VecDeque::from(vec![v]) }
    }

    fn len(&self) -> usize {
        self.array.len()
    }

    // for i in (self.start()..self.end()) {
    //     self.get(i)
    // }
    fn end(&self) -> i32 {
        self.start + (self.len() as i32)
    }

    fn get(&self, n: i32) -> Option<&T> {
        if n >= self.start && n < self.end() {
            self.array.get((n-self.start).try_into().unwrap())
        } else {
            None
        }
    }

    fn insert(&mut self, n: i32, v: T) {
        if n < self.start {
            self.push_front(v);
        } else if n >= self.end() {
            self.push_back(v);
        } else {
            self.array.insert((n-self.start).try_into().unwrap(), v);
        }
    }

    fn push_front(&mut self, v: T) {
        self.start -= 1;
        self.array.push_front(v);
    }

    fn push_back(&mut self, v: T) {
        self.array.push_back(v)
    }
}


#[wasm_bindgen]
#[derive(Debug)]
pub struct Wall {
    //memo: OffsetArray<OffsetArray<BigInt>>,
    memo: HashMap<(i32, i32), BigInt>,
    //memo: BTreeMap<i32,BTreeMap<i32, BigInt>>,
    function: Function,
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum Function {
    Square,
    DeBruijn,
    Rueppel,
    Rook,
    Knight,
}

#[wasm_bindgen]
impl Wall {
    pub fn new(function: Function) -> Self {
        let v = match function {
            Function::Square => square(0),
            Function::Knight => knight(0),
            Function::Rook => rook(0),
            Function::DeBruijn => binary(0),
            Function::Rueppel => rueppel(0),
        };
        //Wall { memo: OffsetArray::new(0, OffsetArray::new(0, v)), function }
        Wall { memo: HashMap::new(), function }
        //Wall { memo: BTreeMap::new(), function }
    }

    fn func(&self, n: i32) -> BigInt {
        match self.function {
            Function::Square => square(n),
            Function::DeBruijn => binary(n),
            Function::Rueppel => rueppel(n),
            Function::Rook => rook(n),
            Function::Knight => knight(n),
        }
    }
    /*
    fn memo_get(&self, m: i32, n: i32) -> Option<BigInt> {
        self.memo.get(m)
            .and_then(|x| x.get(n).cloned())
    }
    */

    /*
    fn memo_insert(&mut self, m: i32, n: i32, v: BigInt) {
        self.memo.entry(m)
            .and_modify(|data| {data.insert(n, v.clone());})
            .or_insert_with(|| {
                let mut new = BTreeMap::new();
                new.insert(n, v);
                new
            });
    }
    */
    /*
    fn memo_insert(&mut self, m: i32, n: i32, v: BigInt) {
        if let Some(inner) = self.memo.array.get_mut(m.try_into().unwrap()) {
            inner.insert(n, v);
        } else {
            self.memo.insert(m, OffsetArray::new(n, v))
        }
    }
    */

    pub fn get(&mut self, m: i32, n: i32) -> BigInt {
        if let Some(v) = self.memo.get(&(m, n)) {
        //if let Some(v) = self.memo_get(m, n) {
            v.clone()
        } else {
            let v = self.get_item(m, n);
            // no point wasting space on really trivial things
            if m > -1 {
                self.memo.insert((m,n), v.clone());
                //self.memo_insert(m, n, v.clone());
            }
            v
        }
    }

    fn get_item(&mut self, m: i32, n: i32) -> BigInt {
        if m < -1 {
            0.into()
        } else if m == -1 {
            1.into()
        } else if m == 0 {
            self.func(n)
        } else if self.get(m-2, n) == 0 {
            let (inside, (top, left, right)) = self.window_check(m, n);
            let g = right - left + 1;
            if inside {
                0.into()
            } else if self.get(m-1, n) == 0 {
                let i = right - n;
                let b = self.get(top+i, left-1);
                let c = self.get(m-1-i, right+1);
                let a = self.get(top-1, left+i);
                let d = b*c/a;
                if (g*(i+1)) % 2 == 0 { d } else { -d }
            } else {
                let i = right - n;
                let d = self.get(m-1, n);
                let e = self.get(top-2, left+i);
                let a = self.get(top-1, left+i);
                let f = self.get(top+i, left-2);
                let b = self.get(top+i, left-1);
                let g = self.get(m-2-i, right+2);
                let c = self.get(m-2-i, right+1);
               
                let rn = self.get(top, right+1);
                let rd = self.get(top+1, right+1);
                let qn = self.get(top+1, left-1);
                let qd = self.get(top, left-1);
                let pn = self.get(top-1, left+1);
                let pd = self.get(top-1, left);
                let tn = self.get(m-1, left-1);
                let td = self.get(m-1, left);
                
                let xn = qn*d.clone()*e*rd.clone();
                let xd = rn.clone()*a*qd;
                let yn = if (i+1)%2 == 0 { pn*d.clone()*f*rd.clone() } else { -pn*d.clone()*f*rd.clone()};
                let yd = pd*rn.clone()*b;
                let zn = if (i+1)%2 == 0 { -tn*rd.clone()*d.clone()*g } else { tn*rd.clone()*d.clone()*g };
                let zd = td*rn*c;
                let res = (xn*yd.clone()*zd.clone() + yn*xd.clone()*zd.clone() + zn*xd.clone()*yd.clone())/(xd*yd*zd);

                res
            }
        } else {
            let a = self.get(m-1, n);
            let b = self.get(m-1, n-1);
            let c = self.get(m-1, n+1);
            let d = self.get(m-2, n);
            ((a.clone()*a-b*c)/d)
        }
    }

    // the function that can exit early
    fn window_check(&mut self, m: i32, n: i32) -> (bool, (i32, i32, i32)) {
        if self.get(m-1, n) != 0 {
            (false, self.get_window(m-2, n))
        } else {
            let mut top = m-2;
            let mut left = n;
            let mut right = n;
            
            while self.get(top-1, n) == 0 {
                top -= 1;
            }

            let d = m - top + 1;
            
            while self.get(top, left-1) == 0 && left + d > n {
                left -= 1;
            }

            while self.get(top, right+1) == 0 && right - d < n {
                right += 1;
            }

            ((right - left + 1) >= d, (top, left, right))
        }
    }

    fn get_window(&mut self, m: i32, n: i32) -> (i32, i32, i32) {
        let mut top = m;
        let mut left = n;
        let mut right = n;
        
        while self.get(top-1, n) == 0 {
            top -= 1;
        }
        
        while self.get(top, left-1) == 0 {
            left -= 1;
        }

        while self.get(top, right+1) == 0 {
            right += 1;
        }

        (top, left, right)

    }
}



#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(&format!("sup {}", s));
}
