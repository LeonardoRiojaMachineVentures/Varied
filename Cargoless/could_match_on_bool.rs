fn main() {
    let x = false;
    let v = true;
    match x {
        false => {println!("Yes can match on bool");},
        true => {println!("Hey");},
    }
    match v {
        false => {println!("Nothing");},
        true => {println!("Hey");},
    }
}