use std::cmp::Ordering;
use std::io;
use rand::Rng; //use is to include libraries 
             //std::io defines the input output library form the standard lib.

fn main(){//main function starts
    println!("Guessing Game, Guess the Number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop{
    println!("Please Input your guess:");
    let mut guess = String::new();  //let is used to declare variables
                                   //mut is to make the variable muttable and variables and
                                  //immutable by default in rust

    io::stdin().read_line(&mut guess).expect("Invalid Input");//io::stdin is refrenceing to input
                                                             //output lib read_line function
                                                            //stores the value of type String to
                                                           //the muttable declared above.
                                                          //expect take the Result value
                                                         //returned by read_line and uses it
                                                        //for error catch.
    //println!("The Secret Number is {secret_number}");
    println!("You have guessed:{guess}");//Prints value in {} guess in our case
                                        //Alternatively: 
                                       //println!("You have guessed:{}",guess);
    let guess : u32 = guess.trim().parse().expect("Expected Integer recived invalid");//Here we
                                                                                     //assign a new
                                                                                    //value to
                                                                                   //guess which is
                                                                                  //of type int(u32)
                                                                                 //trim() is used to
                                                                                //remove extra
                                                                               //white spaces.
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Equal => {
            println!("You have guessed correctly");
            break;}
        Ordering::Greater => println!("Too large"),

    
    }//match case with cmp(compare function) to Order acc. to inqualities
    }



}//main function ends.
