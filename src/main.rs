use std::io;

fn main() {
    let options = vec![
        "View user details",
        "List repositories",
        "Clone repositories",
    ];

    println!("Hello, which action do you want to perform?");

    for (index, option) in options.iter().enumerate() {
        println!("{}. {}", index + 1, option)
    }

    let mut choice = String::new();
    let mut valid_choice: usize;

    while true {
        choice = "".to_string();

        io::stdin()
            .read_line(&mut choice)
            .expect("Error reading input");

        if choice < options.len().to_string() {
            valid_choice = choice.parse().unwrap();
            println!("You have chosed {}", choice);

            break;
        }
        println!("Invalid choice, enter again");
    }
}
