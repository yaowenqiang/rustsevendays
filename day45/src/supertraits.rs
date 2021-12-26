trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn faw_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}, My favorite language is {}, My Git username is {}",
        student.name(),
        student.university(),
        student.faw_language(),
        student.git_username()
        )
}

struct MyStudent {
    name: String,
    age: i32,
}

impl CompSciStudent for MyStudent {
    fn name(&self) -> String {
        self.name
    }
    fn faw_language() -> String {
        "MIT"
    }

    fn git_username() -> String {
        "jack".to_string()
    }
}



fn main() {
    let my_student = MyStudent {name: "jack".to_string(), age: 15,};
    println!("{}", comp_sci_student_greeting(my_student));

}

