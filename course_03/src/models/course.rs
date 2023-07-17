use crate::models::student::Student;

#[derive(Debug)]
pub struct Course {
    pub name: String,
    pub students: Vec<Student>,
}
