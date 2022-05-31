fn main() {
    let today = Weekdays::Monday;

    match today {
        Weekdays::Monday => println!("Here we go!"),
        Weekdays::Wednesday => println!("Hump day"),
        Weekdays::Friday => println!("Yay!"),
        _ => println!("just another day")
    }
}

enum Weekdays {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday
}

