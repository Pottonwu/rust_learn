struct Type1 {
    num : u32,
}
impl Type1{
    fn square(&self) -> u32{
        self.num * self.num
    }
}
struct type2 {
    name:String,
}
impl type2 {
    fn sum(self) -> String {
        self.name + "_students"
    }
}
struct type3{
    num : i32,
}
impl type3 {
    fn div(&self) -> i32 {
        self.num/2
    }
}
enum MyEnum {
    a(Type1),
    b(type2),
    c(type3),
}

fn main() {
    let my_vec = vec![
        MyEnum::a(Type1{num :10}),
        MyEnum::b(type2{name : String::from("peter")}),
        MyEnum::c(type3{num : 100}),
    ];

    for item in my_vec {
       match item {
           MyEnum::a(x) => println!("{}",x.square()),
           MyEnum::b(x) => println!("{}",x.sum()),
           MyEnum::c(x)=> println!("{}",x.div())
       }
    }
}
