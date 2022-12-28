struct Student {
    name: String,
    level: u8,
    remote: bool
}

struct Grades(
    char,
    char,
    char,
    char,
    f32
);

struct Unit;

fn main() {
    // The `mut` keyword lets the variable be changed
    /* let mut a_number = 10; 
    println!("The number is {}.", a_number);

    // Change the value of an immutable variable
    a_number = 15;
    println!("Now the number is {}.", a_number); */
        // Declare first variable binding with name "shadow_num"
    /* let shadow_num = 5;

    // Declare second variable binding, shadows existing variable "shadow_num" 
    let shadow_num = shadow_num + 5; 

    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num = shadow_num * 2; 

    println!("The number is {}.", shadow_num); */

    /* let number: u32 = 14;
    println!("The number is {}.", number); */
    // Addition, Subtraction, and Multiplication
    /*println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);

    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);*/


    /*let character_1: char = 'S';
    let character_2: char = 'f';

    let smiley_face = '😃';

    let string_1 = "miley ";
    
    let string_2: &str = "ace";

    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);*/


    /*let tuple = ('E', 5i32,true);
    println!("Is '{}' the {}the letter of the alphabet? {}", tuple.0,tuple.1,tuple.2);*/

    let user_1 = Student {
        name: String::from("Johs"),
        remote: false,
        level: 5
    };
    let user_2 = Student {
        name: String::from("John"),
        level: 2,
        remote: true
    };

    let mark_1 = Grades('A', 'B', 'C', 'D', 3.14);
    let mark_2 = Grades('B', 'A', 'D', 'A', 3.25);

    print!("name : {}, level {}, remote: {}. Grades : {}, {}, {}, {}. Average : {}\n", user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
    print!("name : {}, level {}, remote: {}. Grades : {}, {}, {}, {}. Average : {}", user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);
}