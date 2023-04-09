use std::io;

fn main() {
    let riddle=" I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I ?";
    let answer="e";
    let mut number_of_trials = 0;
    
    loop {
        println!("{}", riddle);
        let mut user_answer = String::new();
        io::stdin().read_line(&mut user_answer).expect("Failed to read line");
        let user_answer = user_answer.trim();
        
        if user_answer==answer {
            number_of_trials += 1;
            println!("Number of trials: {}", number_of_trials);
            break();
        }    
        else{
            number_of_trials += 1;
        }
    }
    
}
