fn main() {
    //If
    if 1 == 2 {
        println!("Math is broken!");
    } else {
        println!("everything is fine");
    }

    let formal = true;
    let greeting = if formal {
        println!("Good Evening");
    } else {
        println!("Hey, friend");
    };

    let number = 6;

    if number % 4 == 0 {
        println!("number is divible by 4");
    } else if number % 3 == 0 {
        println!("Number is divible by 3");
    } else {
        println!("number is not divisible by 3 or 4");
    }

    //Match :: same with Switch case
    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };
}
