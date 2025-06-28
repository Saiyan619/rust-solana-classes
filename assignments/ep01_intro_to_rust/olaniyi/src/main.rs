use std::io;
use rand::Rng;

fn main(){
//     println!("{}",addition(2, 500));
//     let x:i32=60;
//     println!("this is {}", x);

//     let numbers:[u32;3]=[1,2,3];
//     println!("{:?}", numbers);

//     let mut num:u16 = 0;
//     while num <10{
//         num = num + 1;
//         println!("count to {num}")
//     };

//     let arr:[u16;5] = [1,2,3,4,5];
//     for i in arr{
//         println!("{i}");
//     }

//     let name:&str = "";
//     if name == "Niyi"{
//         println!("Dayo would call me that");
//     }else if name == "Arokoyu"{
//         println!("maxi would call me that")
// }else{
//     println!("i dont know you nigga")
// }

// let arr :[i32; 3] =[1,2,3];
// let arr : [i32; 20]= [];
// let mut arr : Vec<i32> = Vec::new();
// let mut tracker:i32 = 0;

// while tracker < 20{
//     tracker +=1;
//     arr.push(tracker);
// }
//     println!("{:?}", arr);


// for i in arr {
//     if i % 3 == 0{
//         println!("fizz");
//     }
//     else if i % 5 == 0{
//         println!("buzz");
//     }else if i % 3 == 0 && i % 5 == 0{
//         println!("fizzBuzz");
//     }
//     else{
//         println!("{:?}", i);
//     }
// }



// greeter();
// addition();


// calculator();
// suggestion_calculator();

fizz_buzz();
}

// This function generates numbers from 1 to 100 and prints "fizz" for multiples of 3, "buzz" for multiples of 5, and "fizzbuzz" for multiples of both.
    // If a number is not a multiple of 3 or 5, it prints the number itself.
fn fizz_buzz(){
    
    let mut arr_of_numbers = Vec::new(); // Made an empty vector to push in the numbers 1-100
    let mut tracker:i32 = 0;
    // As long as tracker is less than 100, we increment tracker by 1
    while tracker < 100{
         tracker += 1;
        arr_of_numbers.push(tracker);
    }
    // Now we loop through the vector of numbers and check each number
for i in arr_of_numbers{
    if i % 3 == 0 && i % 5 == 0 {
        println!("fizzbuzz");
    }else if i % 3 == 0 {
        println!("fizz");
    }else if i % 5 == 0 {
        println!("buzz");
    }else{
        println!("{:?}", i);
    }
}
}

// fn suggestion_calculator(){
//     let mut input = String::new();
//     println!("Suggester Calculator");
//     println!("Input your number");
//     io::stdin().read_line(&mut input).expect("failed");
//     let mut generator = rand::thread_rng();
//     let rand_num = generator.gen_range(1000..=5000);
//     let input_num: i32= input.trim().parse().expect("wrong input");
//     let calculation = input_num * rand_num;
//     println!("Your suggestion is: {}", calculation);

// }



// fn greeter(){
// let mut name = String::new();
// io::stdin().read_line(&mut name).expect("failed");
// let name = name.trim();
// println!("hello {}", name);
// }


// fn addition(){
//     let mut input1 = String::new();
//     let mut input2 = String::new();
//     println!("+--Addition Calculator--+");
//     println!("First Number");
//     io::stdin().read_line(&mut input1).expect("failed");
//         let num1:i32 = input1.trim().parse().expect("wrong input");

//         println!("Second Number");
//     io::stdin().read_line(&mut input2).expect("failed");
//         let num2:i32 = input2.trim().parse().expect("wrong input");

//     let sum = num1 + num2;
//     println!("your answer by addition is {}", sum);
// }
// fn subtraction(){
//     let mut input1 = String::new();
//     let mut input2 = String::new();
//     println!("+--Subtraction Calculator--+");
//     println!("First Number");
//     io::stdin().read_line(&mut input1).expect("failed");
//         let num1:i32 = input1.trim().parse().expect("wrong input");

//         println!("Second Number");
//     io::stdin().read_line(&mut input2).expect("failed");
//         let num2:i32 = input2.trim().parse().expect("wrong input");

//     let minus = num1 - num2;
//     println!("your answer by subtraction is {}", minus);
// }
// fn division(){
//     let mut input1 = String::new();
//     let mut input2 = String::new();
//     println!("+--division Calculator--+");
//     println!("First Number");
//     io::stdin().read_line(&mut input1).expect("failed");
//         let num1:i32 = input1.trim().parse().expect("wrong input");

//         println!("Second Number");
//     io::stdin().read_line(&mut input2).expect("failed");
//         let num2:i32 = input2.trim().parse().expect("wrong input");

//     let divided = num1 / num2;
//     println!("your answer by division is {}", divided);
// }
// fn multiplication(){
//     let mut input1 = String::new();
//     let mut input2 = String::new();
//     println!("+--multiplication Calculator--+");
//     println!("First Number");
//     io::stdin().read_line(&mut input1).expect("failed");
//         let num1:i32 = input1.trim().parse().expect("wrong input");

//         println!("Second Number");
//     io::stdin().read_line(&mut input2).expect("failed");
//         let num2:i32 = input2.trim().parse().expect("wrong input");

//     let multiply = num1 * num2;
//     println!("your answer by multiplication is {}", multiply);
// }

// fn calculator(){
//     let mut input = String::new();
//     println!("+--Calculator--+");
//     println!("Enter 1 for Addition");
//     println!("Enter 2 for Subtraction");
//     println!("Enter 3 for Division");
//     println!("Enter 4 for Multiplication");
//     io::stdin().read_line(&mut input).expect("failed");
//     let choice: i32 = input.trim().parse().expect("wrong input");

//     match choice {
//         1 => addition(),
//         2 => subtraction(),
//         3 => division(),
//         4 => multiplication(),
//         _ => println!("Invalid choice, please try again."),
//     }

//     println!("Thank you for using the calculator!");
// }






