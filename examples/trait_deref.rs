// 为函数或方法参数自动解引用

fn main() {
    std();
}

// 内置标准库实现
fn std() {
    // 在堆上建个字符串, Box 引用类型
    let x = Box::new(String::from("hello"));

    // 因为标准库中的 Box 实现了 Deref trait, 所以 println! 输出的是字符串值, 而不是 x 指针本身
    println!("hello: {}", x);

    // 如果不适用 Deref, 则手动转换方法如下
    // 不使用Deref强制转换
    let x = Box::new(String::from("hello"));
    let s: String = *x; // 将Box<String>解引用为String
    let s: &str = &s[..]; // 使用`&`和`[..]`获取整个String的字符串切片
    println!("hello: {}", s);
}

// 自定义实现
#[allow(unused)]
fn custom() {
    let s1 = String::from("hello");
    let s2: String = "hello".to_string();
}
