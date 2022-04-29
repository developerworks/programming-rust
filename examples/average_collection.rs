// https://rustwiki.org/zh-CN/book/ch17-01-what-is-oo.html
use rand::{thread_rng, Rng};

// 添加/删除元素是, 自动计算集合中元素的平均值

#[derive(Default)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
    total: i32,
}
impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection::default()
    }
    // 添加元素
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    // 删除元素
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    // 获取平均值
    pub fn average(&self) -> f64 {
        self.average
    }

    // 集合长度
    pub fn len(&self) -> usize {
        self.list.len()
    }

    // 是否为空
    pub fn is_empty(&self) -> bool {
        self.list.len() == 0
    }

    // 更新平均值
    fn update_average(&mut self) {
        // let total: i32 = self.list.iter().sum();
        self.total = self.list.iter().sum();
        self.average = self.total as f64 / self.list.len() as f64;
        
    }
}
fn main() {
    let mut ac = AveragedCollection::new();
    let mut rng = thread_rng();
    
    for _ in 0..100 {
        let idx = rng.gen_range(10..1000);
        ac.add(idx);
        println!("add: {}, avg: {}, total: {}", idx, ac.average(), ac.total);
    }
}
