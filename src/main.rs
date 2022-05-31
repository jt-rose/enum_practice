use std::fmt;

fn main() {
    let today = Weekdays::Monday(Class{ course: String::from("Rust"), teacher: String::from("Herbert Wolverson"), grade: 4.5});
    //let tomorrow = Weekdays::Thursday(Work{ hours: 5.00, pay: 75.00, shift: Shift::Night});

    match today {
        Weekdays::Monday(class) => print_class_info(&class),
        Weekdays::Tuesday(work) => print_work_info(&work),
        Weekdays::Wednesday(class) => print_class_info(&class),
        Weekdays::Thursday(work) => print_work_info(&work),
        Weekdays::Friday(class) => print_class_info(&class),
    }
}

fn print_class_info(class_info: &Class) {
    println!("course: {} taught by {}", class_info.course, class_info.teacher);
}

fn print_work_info(work_info: &Work) {
    println!("earned ${} over {} hours during the {} shift", work_info.pay, work_info.hours, work_info.shift.to_string());
}

enum Weekdays {
    Monday(Class),
    Tuesday(Work),
    Wednesday(Class),
    Thursday(Work),
    Friday(Class)
}

struct Class {
    course: String,
    teacher: String,
    grade: f64
}

enum Shift {
    Early,
    Afternoon,
    Night
}

impl fmt::Display for Shift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shift::Early => write!(f, "early"),
            Shift::Afternoon => write!(f, "afternoon"),
            Shift::Night => write!(f, "night"),
        }

    }
}

struct Work {
    hours: f64,
    pay: f64,
    shift: Shift
}

