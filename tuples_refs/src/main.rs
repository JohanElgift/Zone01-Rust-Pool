#[derive(Debug)]
pub struct Student(i32, String, String);

pub fn id(student: &Student) -> i32 {
    student.0
}

pub fn first_name(student: &Student) -> String {
    student.1.clone()
}

pub fn last_name(student: &Student) -> String {
    student.2.clone()
}

fn main() {
    let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
    println!("Student: {:?}", student);
    println!("Student first name: {}", first_name(&student));
    println!("Student last name: {}", last_name(&student));
    println!("Student Id: {}", id(&student));
}
