use std::collections::HashMap;
struct Student {
    name:String,
    sex:String,
    age:u8,
    clubs:String,
    classes:u8,
    courses:String,
}

struct School {
    students:HashMap<u32,Student>,
}
impl School {
    fn new() ->School {
        School {
            students: HashMap::new(),
        }
    }
    fn creat(&mut self,id:u32,name:String,sex:String,age:u8,clubs:String,classes:u8,coures:String){
        let s:Student=Student { name: (name), sex: (sex), age: (age), clubs: (clubs), classes: (classes), courses: (coures) };
        self.students.insert(id, s);
    }
    fn delete(&mut self,id:u32){
        if let Some(x) =  self.students.remove(&id){
            println!("删除成功");
        }
        else {
            println!("删除失败")
        }

    }
    fn retrieve(&self,id:u32) -> Option<&Student>{
        if let Some(x) = self.students.get(&id){
            println!("name:{}",x.name);
            println!("age:{}",x.age);
            println!("sex:{}",x.age);
            println!("clubs:{}",x.clubs);
            println!("classes:{}",x.classes);
            println!("coures:{}",x.courses);
        }
        else {
            println!("未查询到学生");
        }
        self.students.get(&id)
    }
    fn update(&mut self,id:u32,name:Option<String>,sex:Option<String>,age:Option<u8>,classes:Option<u8>,clubs:Option<String>,coures:Option<String>){
        if let Some(x) = self.students.get_mut(&id){
            if let Some(name_t) = name{
                x.name = name_t;
            }
            if let Some(sex_t) = sex{
                x.sex = sex_t;
            }
            if let Some(age_t) = age{
                x.age = age_t;
            }
            if let Some(classes_t) = classes{
                x.classes = classes_t;
            }
            if let Some(clubs_t) = clubs{
                x.clubs = clubs_t;
            }
            if let Some(coures_t) = coures{
                x.courses = coures_t;
            }
        }
    }   
}

fn main() {
    let mut School_1 : School = School::new();
    School_1.creat(01,"mike".to_string(), "男".to_string(), 21, "睡觉社".to_string(), 01, "程序设计".to_string());
    School_1.creat(02,"alice".to_string(), "女".to_string(), 19, "睡觉社".to_string(), 01, "程序设计".to_string());
    println!("-------------------------查----------------------------------");
    School_1.retrieve(01);
    println!("-------------------------改----------------------------------");
    School_1.update(01, Some("小丽".to_string()), Some("无性".to_string()), Some(20), Some(02), None, None);
    School_1.retrieve(01);
    println!("-------------------------删----------------------------------");
    School_1.delete(01);
    School_1.retrieve(01);
}
