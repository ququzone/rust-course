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

#[derive(Debug)]
pub struct Class {
    pub grade: Grade,
    pub num: u32,
    pub students: Vec<Student>,
}
