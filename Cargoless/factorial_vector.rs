fn factorial_vector(x : u8) -> Vec<usize> {
    assert!(x != 0, "For this function, do not accept argument equal to zero");
    //let mut ans = Vec::with_capacity(x as usize);
    //let ans = (1..(x + 1)).filter(|x| x % 2 == 0).map(|x| x.pow(2)).collect::<Vec<_>>();
    let mut p = 1usize;
    let x = x as usize;
    (2..(x + 1)).map(|i| { p *= i; p }).collect::<Vec<usize>>()
}
fn combinations(x : u8) -> Vec<Vec<usize>> {
    assert!(x != 0, "For this function, do not accept argument equal to zero");
    let v = factorial_vector(x).rev();
    let ans = Vec::with_capacity(v.first());
    let inner = Vec::with_capacity(x as usize);
}
fn main() {
    //let v = factorial_vector(0);
    //println!("{:?}", v);
    let v = factorial_vector(1);
    println!("{:?}", v);
    let v = factorial_vector(2);
    println!("{:?}", v);
    let v = factorial_vector(3);
    println!("{:?}", v);
    let v = factorial_vector(4);
    println!("{:?}", v);
    let v = combinations(3);

}