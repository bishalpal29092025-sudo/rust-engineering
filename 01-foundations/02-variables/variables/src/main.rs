
const OWNER: &str = "Bishal Pal";


fn main() {

    let name = "Bishal Pal";
    let age = 24;
    let country = "India";
    let state = "West Bengal";
    let city = "Bankura";
    let university = "Adamas University";
    let course = "MCA";
    let semester = "4th";
    let language = "Rust";
    let goal = "Full Stack Solana Developer";


    println!("=================================");
    println!("      Student Information        ");
    println!("=================================");
    println!("Name      : {}", name);
    println!("Age       : {}", age);
    println!("Country   : {}", country);
    println!("State     : {}", state);
    println!("City      : {}", city);
    println!("University: {}", university);
    println!("Course    : {}", course);
    println!("Semester  : {}", semester);
    println!("Language  : {}", language);
    println!("Goal      : {}", goal);

    let mut age = 24;
    println!("Age: {}", age);

    age = 25;
    println!("Age: {}", age);



    // Shadowing
    let age = 24;
    println!("Age: {}", age);
    let age = 25;
    println!("Age: {}", age);



    let input = "42";
    let input = input.parse::<i32>().unwrap();
    println!("{}", input);


    let number = 5;
    let number = number + 5;
    let number = number * 2;
    println!("{}", number);


    let input = "41";
    let parse_input = input.parse::<i32>().unwrap();
    let double_input = parse_input * 2;

    println!("{}", input);
    println!("{}", parse_input);
    println!("{}", double_input);

    let input = "41";
    let input = input.parse::<i32>().unwrap();
    let input = input * 2;

    println!("Input is {}", input);


    let mut count = 10;
    println!("Count is: {}", count);
    count = 20;
    println!("Count is after change the mutable value: {}", count);
    let count = count + 5;
    println!("Count is now {}", count);


    const MAX_USERS: u32 = 100;
    println!("Maximum users at a time: {}", MAX_USERS);

    const APP_NAME: &str = "Rust Engineering";
    println!("Application Name is {}", APP_NAME);

    println!("Owner name {}", OWNER);

    const PI_VALUE: f64 = 3.147;
    println!("Value of PI is {}", PI_VALUE);
}
