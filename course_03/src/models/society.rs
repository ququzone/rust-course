use crate::models::student::Student;

#[derive(Debug)]
pub struct Society {
    pub name: String,
    pub members: Vec<Student>,
}

impl Society {
    pub fn add_student(&mut self, student: Student) {
        self.members.push(student);
    }
}
