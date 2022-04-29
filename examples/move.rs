fn main() {
    let x = 0;
    {
        let _y = x;
    }
    println!("x = {}", x);
}