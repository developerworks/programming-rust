// https://blog.frognew.com/2020/07/rust-default-trait.html
#[derive(Debug)]
pub enum PayStatus {
    Unpaid,
    Paid,
}

impl Default for PayStatus {
    fn default() -> Self {
        PayStatus::Unpaid
    }
}
#[derive(Default, Debug)]
#[allow(unused)]
pub struct Price {
    original_price: f64,
    discounted_price: f64,
    name: String,
    
}
#[derive(Default, Debug)]
#[allow(unused)]
pub struct Order {
    id: i64,
    pay_status: PayStatus,
    price: Price,
}

#[derive(Default, Debug)]
struct TupleWithDefault(i32, i32, i32);

fn main() {
    let order1 = Order::default();
    let order2 = Order {
        id: 100,
        // 解构更新语法
        ..Order::default()
    };
    println!("order1 = {:?}", order1);
    println!("order2 = {:?}", order2);
    println!("default string: {:?}", String::default());
    println!("default tuple: {:?}", TupleWithDefault::default());
}
