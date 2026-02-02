fn main () {
    let evil = make_static(&vec![0; 1 << 20]);
    println!("{:?}", evil);
}
