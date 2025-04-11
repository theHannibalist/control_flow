fn main() {

    // if and else :
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let numbr = if condition { 5 } else { 6 };
    println!("the number is {numbr}");

    // let's try loops here :


        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 20 {
                break counter * 2;
            }
        };
    println!("the result is {result}");

    // while :

    let mut while_number = 100;

    while while_number != 0 {
        println! ("{while_number} !");
        while_number -= 1;
    }
    println!("STOP!");

    // for :
    let a = [10 , 20 , 30 , 40 , 50];

    for element in a {
        println! ("the index is {element}");
    }
    println! ("\n");
    for i in (1..4).rev() { // rev() will reverse the numbers
        println!("{i} !");
    }

    println! ("STOP!");
}
