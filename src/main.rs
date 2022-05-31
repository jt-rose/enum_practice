fn main() {
    //let today = Weekdays::Monday(Class{ course: String::from("Rust"), teacher: String::from("Herbert Wolverson"), grade: 4.5});
    let tomorrow = Weekdays::Thursday(Work{ hours: 5.00, pay: 75.00, shift: Shift::Night});

    match tomorrow {
        Weekdays::Monday(class) => println!("time for school"),
        Weekdays::Wednesday(class) => println!("time for school"),
        Weekdays::Friday(class) => println!("time for school"),
        _ => println!("time for work")
    }
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

struct Work {
    hours: f64,
    pay: f64,
    shift: Shift
}

