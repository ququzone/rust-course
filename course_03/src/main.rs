use course_03::models::class::{Class, Grade};
use course_03::models::course::Course;
use course_03::models::society::Society;
use course_03::models::student;
use course_03::models::student::Student;

fn main() {
    let mut all_students: Vec<Student> = vec![];

    let bob = Student::new(String::from("Bob"), 10);
    let alice = Student::new(String::from("Alice"), 11);
    let mike = Student::new(String::from("Mike"), 9);

    // init all students
    all_students.push(bob);
    all_students.push(alice);
    all_students.push(mike);
    all_students.push(Student::new(String::from("Tom"), 8));

    // ------ Student CURD begin -----
    // query student with name `foo`
    let foo = student::find_first_by_name(&all_students, String::from("foo"));
    assert!(foo.is_none(), "find foo student");
    let bob = student::find_first_by_name(&all_students, String::from("Bob"));
    assert_eq!(bob.unwrap().0.age, 10);
    let index = bob.unwrap().1;
    // increase bob age
    let _ = &mut (all_students[index]).increase_age();

    // remove foo from all students
    assert!(
        !student::remove_first_by_name(&mut all_students, String::from("foo")),
        "remove foo student"
    );
    // remove tom from all students
    assert!(
        student::remove_first_by_name(&mut all_students, String::from("Tom")),
        "can't remove tom student"
    );
    assert_eq!(all_students.len(), 3);
    println!("{:?}", all_students);
    // ------ Student CURD end -----

    // ------ Class CURD begin -----
    let mut class1 = Class {
        grade: Grade::ONE,
        num: 1,
        students: vec![],
    };
    let bob = student::find_first_by_name_clone(&all_students, String::from("Bob")).unwrap();
    class1.add_student(bob);
    let alice = student::find_first_by_name_clone(&all_students, String::from("Alice")).unwrap();
    class1.add_student(alice);
    println!("{:?}", class1);
    class1.remove_student(String::from("Alice"));
    println!("{:?}", class1);
    // ------ Class CURD end -----

    let mut couse1 = Course {
        name: String::from("English"),
        students: vec![],
    };
    let bob = student::find_first_by_name_clone(&all_students, String::from("Bob")).unwrap();
    couse1.add_student(bob);
    println!("{:?}", couse1);

    let mut society = Society {
        name: String::from("Game"),
        members: vec![],
    };
    let bob = student::find_first_by_name_clone(&all_students, String::from("Bob")).unwrap();
    society.add_student(bob);
    println!("{:?}", society);
}
