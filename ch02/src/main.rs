
struct Person {
    name: &'static str,
    age: u64,
}

impl Person {
    fn say(&self) -> String {
        format!("person said!")
    }
}

fn main() {
    my_fn();
    const ACCOUNT: u64 = 3410;
    println!("{}", ACCOUNT);
    println!("sum: {}", sum(33, 66));

    let p = Person{
        name: "test",
        age: 10,
    };
    println!("{}", p.say());

    let mut v: Vec<i32> = vec![1,2,3];
    v.push(5);
    for i in v {
        println!("{}", i)
    }
}

fn my_fn() {
    println!("this is from function");
}

fn sum(a: u64, b: u64) -> u64 {
    a + b
}
