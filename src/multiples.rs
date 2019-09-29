use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("No arguments passed. Must pass an integer to find multiples of and a max \
            multiple to stop at");
        }

        2 => {
            println!("Insufficient arguments passed. Must pass a positive integer to find multiples \
            of and a max multiple to stop at (also a positive integer)");
        }

        _ => {
            let target_nb = args[1]
                .trim()
                .parse()
                .expect(&format!("Invalid input. Expected a positive integer, received: {}",
                                 args[1]));

            let ceiling = args[2]
                .trim()
                .parse()
                .expect(&format!("Invalid input. Expected a positive integer, received: {}",
                                 args[1]));
            print_multiples(target_nb, ceiling);
        }
    }
}

fn print_multiples(target_nb: i64, ceiling: i64) {
    println!("Determining multiples for {}", target_nb);
    let mut multiple = target_nb;
    while multiple <= ceiling {
        println!("{}", multiple);
        multiple += target_nb;
    }
}