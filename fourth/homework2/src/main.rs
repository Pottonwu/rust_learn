trait dive {
    fn div(&self) -> u32;
    
}
struct type1 {
    num :u32
}
struct type2 {
    num :u32
}
struct type3 {
    num :u32
}
impl dive for type1 {
    fn div(&self) -> u32 {
        self.num/2
    }
}
impl dive for type2 {
    fn div(&self) -> u32 {
        self.num/2
    }
}
impl dive for type3 {
    fn div(&self) -> u32{
        self.num/2
    }
    
}
fn main() {
    let Myvec:Vec<Box<dyn dive>> = vec![
        Box ::new(type1{num : 10000}),
        Box::new(type2{num : 500}),
        Box::new(type3{num : 10}),
    ];
    for item in Myvec{
        println!("{}",item.div());
    }
}
