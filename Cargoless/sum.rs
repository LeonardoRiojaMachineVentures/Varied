enum Paren {
    Open,
    Close,
}
impl Paren {
    fn validate(x : Vec<Paren>) -> Result<bool, ()> {
        let mut count = 0usize;
        for i in x.iter() {
            match i {
                Paren::Open => {
                    count = match count.checked_add(1) {
                        Some(a) => {a},
                        None => {return Err(());},
                    }
                },
                Paren::Close => {
                    count = match count.checked_sub(1) {
                        Some(a) => {a},
                        None => {return Ok(false);},
                    }
                },
            }
        }
        return Ok(count == 0usize);
    }
}

/*
set n &Natural
let k &Natural
: &Natural::LessThanEq(n k)
$ Set::is_finite(n)
let a &Natural
let b &Natural
let c &Natural
: &Natural::GreaterThanEq(a b)
: &Natural::GreaterThanEq(b c)
$ &Natural::GreaterThanEq(a c)
$ Set::order_is_transitive(set &Natural &Natural::GreaterThanEq)
$ Set::order_is_transitive(n &Natural::GreaterThanEq)

let p SortedArray(n GreaterThanEq)

enum Char
    ClosingP
    OpeningP
    ClosingA
    OpeningA
impl Char
    validate(x let [Char]) (let Boolean)

*/
enum Paren {
    CurlyOpen,
    CurlyClose,
    SquareOpen,
    SquareClose,
}
impl Paren {
    fn validate(x : Vec<Paren>) -> bool {
        let mut left = 0;
        let mut right = 1;
        fn is_end(left : Option<&Paren>, right : Option<&Paren>) -> bool {

        }
        fn is_match(left : &Paren, right : &Paren) -> bool {
            use crate::Paren::*;
            match (left, right) {
                (CurlyOpen, CurlyClose) => true,
                (SquareOpen, SquareClose) => true,
                _ => false,
            }
        }
    }
}


fn pair_sum_sequence(n : u8) -> u8 {
    let mut sum = 0u8;
    for i in 0..n {
        sum += pair_sum(i, i + 1);
    }
    return sum;
}
fn pair_sum(a : u8, b : u8) -> u8 {
    return a + b
}

fn sum(x : u8) -> u8 {
    if x == 0 {
        return 0;
    }
    return (x + sum(x - 1));
}
fn main() {
    println!("{}", pair_sum_sequence(5));
    println!("{}", sum(5));
    println!("{}", what(5));
}
fn what(n : u8) -> u8 {
    if n <= 1 {
        return 1;
    }
    return what(n - 1) + what(n - 1);
}
fn sum_and_product(v : &[u8]) -> () {
    let mut sum = 0u8;
    let mut product = 1;
    for i in v {
        sum += i;
        product *= i;
    }

}