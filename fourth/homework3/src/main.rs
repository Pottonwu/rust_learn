use std::ops::Add;
struct student{
    name : String,
    age : u8,
}
impl Add for student{
    type Output = student;
    fn add(mut self, rhs: student) -> student {
        self.name = self.name + &rhs.name;
        self.age = self.age + rhs.age;
        self

    }
}
fn main() {
    let x :student = student { name: String::from("peter"), age: 20 };
    let y :student = student { name: String::from("Mike"), age: 21 };
    let z = x + y;
    println!("name:{},age:{}",z.name,z.age);
}
