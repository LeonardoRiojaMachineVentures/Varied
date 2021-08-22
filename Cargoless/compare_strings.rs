/*
enum Boolean
    True
    False
enum Char
impl Char
    decide(x let [Char] y let [Char]) (let Boolean)
        mut s [&N 256] = [0; 256]
        match &N::cmp(x.len() y.len())
            Ordering::Greater Ordering::Less
                return(let Boolean::False)
            _
        
        let x Char
        let y &u8
        $ &N::eq(y Char::to_u8(x))
        del x
        del y
        
*/

fn decide2(x : Vec<u8>, y : Vec<u8>) -> bool {
    use std::cmp::Ordering;
    let mut s = [0isize; 256];
    match x.len().cmp(&y.len()) {
        Ordering::Equal => {

        },
        _ => {
            return false;
        }
    }
    for i in 0..(x.len()) {
        s[x[i] as usize] = s[x[i] as usize].checked_add(1).unwrap();
        s[y[i] as usize] = s[y[i] as usize].checked_sub(1).unwrap();
    }
    for i in s.iter() {
        if i != &0isize {
            return false;
        }
    }
    return true;   
}
fn decide(x : Vec<u8>, y : Vec<u8>) -> bool {
    let mut s = [0usize; 256];
    for i in x.iter() {
        s[*i as usize] = s[*i as usize].checked_add(1).unwrap();
    }
    println!("{:?}", s);
    for i in y.iter() {
        s[*i as usize] = match s[*i as usize].checked_sub(1) {
            None => {
                return false;
            },
            Some(a) => {
                a
            },
        }
    }
    println!("{:?}", s);
    for i in s.iter() {
        if i != &0usize {
            return false;
        }
    }
    println!("{:?}", s);
    return true;
}
fn main() {
    println!("{:?}", decide2((b"iceman").to_vec(), (b"cinema").to_vec()));
    println!("{}", decide((b"iceman").to_vec(), (b"cinema").to_vec()));
}