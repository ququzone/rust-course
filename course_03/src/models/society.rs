use crate::models::student::Student;

#[derive(Debug)]
pub struct Society {
    pub name: String,
    pub members: Vec<Student>,
}
