struct Value {
    a : usize,
    b : usize,
}
/*
struct Value
    a &Natural
    b &Natural
enum Ordering
    Less
    Greater
    Equal
impl &Natural
    cmp(x let &Natural y let &Natural) (let Ordering)
        match x
            Zero
                match y
                    Zero
                        return(let Ordering::Equal)
                    S(_)
                        return(let Ordering::Less)
            S(a)
                match y
                    Zero
                        return(let Ordering::Greater)
                    S(b)
                        return(&Natural::cmp(a b))
impl Value
    compare(self let Value x let Value) (let Ordering)
        match &Natural::cmp(self.a x.a)
            Ordering::Equal
                return(&Natural::cmp(self.b x.b))
            s
                return(s)
*/
use std::cmp::Ordering;
impl Value {
    fn compare(self, x : Value) -> Ordering {
        //use crate::Ordering::*;
        match self.a.cmp(&(x.a)) {
            Ordering::Equal => {
                return self.b.cmp(&(x.b));
            },
            s => {
                return s;
            },
        }
    }
    fn new(a : usize, b : usize) -> Value {
        Value {
            a,
            b,
        }
    }
}
fn main() {
    let s = Value::new(12, 33);
    let a = Value::new(213, 23);
    println!("{:?}", s.compare(a));
}

