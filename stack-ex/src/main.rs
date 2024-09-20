const ARR_SZ: usize = 65536;
fn main() {
    let v: [u8; ARR_SZ] = [1; ARR_SZ];
    let first = &v[0];
    println!("The first element is: {first}");
}
