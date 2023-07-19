#[derive(Debug, Clone)]
pub struct Student {
    pub num: u32,
    pub name: String,
    pub age: u32,
}

static mut NUM: u32 = 1;

impl Student {
    pub fn new(name: String, age: u32) -> Student {
        Student {
            num: unsafe {
                let tmp = NUM;
                NUM += 1;
                tmp
            },
            name,
            age,
        }
    }

    pub fn increase_age(&mut self) -> u32 {
        self.age += 1;
        self.age
    }

    pub fn change_name(mut self, name: String) -> String {
        self.name = name;
        self.name
    }
}

pub fn find_first_by_name(students: &Vec<Student>, name: String) -> Option<(&Student, usize)> {
    if let Some(index) = students.iter().position(|s| s.name == name) {
        return Some((&students[index], index));
    }
    None
}

pub fn find_first_by_name_clone(students: &Vec<Student>, name: String) -> Option<Student> {
    if let Some(index) = students.iter().position(|s| s.name == name) {
        return Some(students[index].clone());
    }
    None
}

pub fn remove_first_by_name(students: &mut Vec<Student>, name: String) -> bool {
    let index = students.iter().position(|s| s.name == name);
    match index {
        Some(_) => {
            students.remove(index.unwrap());
            true
        }
        None => false,
    }
}
