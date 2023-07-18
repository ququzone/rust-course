use course_03::models::student;
use course_03::models::student::Student;

fn main() {
    let mut all_students: Vec<Student> = vec![];

    // init all students
    all_students.push(Student::new(String::from("Bob"), 10));
    all_students.push(Student::new(String::from("Alice"), 11));
    all_students.push(Student::new(String::from("Mike"), 9));
    all_students.push(Student::new(String::from("Tom"), 8));

    // query student with name `foo`
    let foo = student::find_first_by_name(&all_students, String::from("foo"));
    assert!(foo.is_none(), "find foo student");
    let bob = student::find_first_by_name(&all_students, String::from("Bob"));
    assert_eq!(bob.unwrap().0.age, 10);
    let index = bob.unwrap().1;
    // increase bob age
    let _ = &mut(all_students[index]).increase_age();

    // remove foo from all students
    assert!(!student::remove_first_by_name(&mut all_students, String::from("foo")), "remove foo student");
    // remove tom from all students
    assert!(student::remove_first_by_name(&mut all_students, String::from("Tom")), "can't remove tom student");
    assert_eq!(all_students.len(), 3);

    println!("{:?}", all_students);
}
