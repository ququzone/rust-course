use crate::models::student::Student;

#[derive(Debug, Clone)]
pub struct Course {
    pub name: String,
    pub students: Vec<Student>,
}

impl Course {
    pub fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }
}
