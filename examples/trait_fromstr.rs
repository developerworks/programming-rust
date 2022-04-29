use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}
// trim_matches 删除字符串两次指定的N个字符
impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')') // 删除,匹配
            .split(',') // 切分
            .collect(); // 收集
        let x = coords[0].parse::<i32>()?;
        let y = coords[1].parse::<i32>()?;
        Ok(Point { x, y })
    }
}
fn main() {
    let p1 = Point::from_str("(2,4)");
    let p2 = "(4,2)".parse::<Point>();
    println!("{:?} {:?}", p1.unwrap(), p2.unwrap());
}
