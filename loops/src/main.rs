fn main() {
    //"Loop" loop
    let mut i = 1;

    let something = loop {
        i *= 2;

        if i > 100 {
            break i;
        }
    };

    assert_eq!(something, 128);

    //While loop
    let mut counter = 0;

    while counter < 10 {
        println!("Hello");
        counter = counter + 1;
    }

    // For loop
    for item in 0..5 {
        println!("{}", item * 2);
    }
}

// 0041646729
