// 狗
#[allow(unused)]
struct Dog {
    pub name: String,
    noise: String,
}
// 猫
#[allow(unused)]
struct Cat {
    pub name: String,
    noise: String,
}

impl Dog {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}
impl Cat {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

// 动物抽象接口
trait Animal {
    fn make_noise(&self);
}

// 接口继承
impl Animal for Dog {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}
impl Animal for Cat {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

// 单太 - Dog
fn static_make_noise(dog: &Dog) {
    dog.make_noise();
}

/// 多态/动态分发 - Animal: Dog || Cat || Or possible any other animals
/// `&dyn Animal` 类似 Java 中的接口参数, 实际类型可以是任意 实现了特定接口的所有子类型
/// 类似的, 在 Rust 中需要明确的指明一个 Trait 是动态分发的
fn dynamic_make_noise(animal: &dyn Animal) {
    animal.make_noise();
}

fn main() {
    let creature = Dog {
        name: String::from("Papi"),
        noise: String::from("旺旺"),
    };
    let cat = Cat {
        name: String::from("Mimi"),
        noise: String::from("瞄"),
    };
    static_make_noise(&creature);
    dynamic_make_noise(&creature);
    dynamic_make_noise(&cat);
}
