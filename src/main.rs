use std::io;

fn check_data(){
    // `mut` help to identify the variable as mutable one
    // By default all variable as set to unmutable
    let mut x: u32 = 5;
    println!("The old no is {x}");
    x = 6;
    println!("The new no is {x}");

    // const variable name shd be a upper case
    const PI: u32 = 22/7;
    println!("The value of π : {PI}");

    let a = "outer";
    {
        let a = "inner";
        println!("From {a}");
    }
    println!("From {a}");

    // len of mutable
    // creating two variable of same name but different type
    // It will keep the latest one
    let spaces = "     ";
    let spaces = spaces.len();
    println!("The length of string : {spaces}");

    // // len of mutable
    // // creating two variable of same name but different type
    // // this shd throw error
    // let mut spaces = "     ";
    // spaces = spaces.len();
    // println!("The length of string : {spaces}");

}

fn interger_data_type(){

    let mut guess = String::new();
    println!("enter the value");
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    // convert string of guess to int
    // using same variable twice is allowed and this is known as shadowing
    let guess: u8 = match guess.trim().parse() {
        Ok(num) => num,
        Err(error) =>0,
    };

    // let guess: u8 = "258".parse().expect("Not a number!");
    // without the u32 this program will throw error
    println!("guess {guess}");

    let a : u8 = 8 - 5;
    println!("a : {a}")

}

fn floating_int(){
    let a = 3.2;
    let b = 5.1;
    println!(" subraction {}", a - b);
    println!(" + {}", 5 + 2);
    println!(" + {}", 5 - 2);
    println!(" + {}", 5.0 * 2.1);
    println!(" + {}", 5.5 / 3.3);
    
}

fn bool_type(){
    let a: bool = true;
    println!("a is {a}");
}

fn scalar_data_types(){
    interger_data_type();
    floating_int();
    bool_type();
    chat_type();
}

fn tuple_type(){
    let a: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:#?}",a);// # is used to pretify the output
    let (x,y,z) = a;
    println!("x {x} y {y} z{z}");
    // using period (.)
    println!("0 {} 1 {} 2{}", a.0,a.1,a.2);

}

fn print_loop(data: &[i32]){
    for i in data.iter(){
        println!("{i}")
    }
}

fn array_type(){
    let a = [1,2,3];
    // println!("index 0 {}",a[0]);
    print_loop(&a);
    println!("--------------------------");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    print_loop(&a[1..3]);
    println!("--------------------------");
    let a = [3; 5];
    print_loop(&a[0..2]);
    println!("--------------------------");
    
}

fn compound_types(){
    // tuple_type();
    array_type();
}

fn data_types(){
    // scalar_data_types();
    compound_types();
}

fn chat_type(){
    let a = "a";
    println!("a is {a}")
}


fn main() {
    data_types()
}
