struct Counter {
    count: usize,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    // we will be counting with usize
    type Item = usize;

    // next() 是唯一要实现的方法, 每调用一次 next() 生成一个返回值
    fn next(&mut self) -> Option<Self::Item> {
        // 计数自增
        self.count += 1;

        // 迭代结束条件
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
fn main() {
    // 创建一个实现了迭代器的 Counter 实例
    let counter = Counter::new();
    // 循环, 迭代
    for i in counter {
        println!("counter: {}", i);
    }
}

#[allow(unused)]
fn iter_ways() {

    #[rustfmt::skip]
    fn iterways() {
        // 不同的迭代方式
        for _ in (0..).take(10)             {}         // 范围 等价于 (0..i32::MAX).take(10)
        for _ in (0..i32::MAX).take(10)     {}
        for _ in (0..2147483647).take(10)   {}
        for _ in [0; 10]                    {}         // 初始值全 0 数组
        for _ in [0..10]                    {}         // 范围数组
    }
}
