use crate::models::student::Student;

#[derive(Debug)]
pub enum Grade {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
}

impl Grade {
    fn next(&self) -> Self {
        use Grade::*;
        match *self {
            ONE => TWO,
            TWO => THREE,
            THREE => FOUR,
            FOUR => FIVE,
            FIVE => SIX,
            SIX => SIX, // can't do next??
        }
    }
}

#[derive(Debug)]
pub struct Class {
    pub grade: Grade,
    pub num: u32,
    pub students: Vec<Student>,
}

impl Class {
    pub fn increase_grade(&mut self) {
        self.grade = self.grade.next()
    }

    pub fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    pub fn remove_student(&mut self, name: String) -> bool {
        if let Some(index) = self.students.iter().position(|s| s.name == name) {
            self.students.remove(index);
            return true;
        }
        false
    }
}
