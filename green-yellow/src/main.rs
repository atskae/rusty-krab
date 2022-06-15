use rand::Rng;

/// Returns a 7-character string containg the results of the guess
/// The string contains 4 result characters, separated by a space
fn calc_green_and_yellow(guess: &[i32], secret: &[i32]) -> String {
    let mut modified_secret: [i32; 4] = [0, 0, 0, 0];
    modified_secret.clone_from_slice(&secret);
    
    let mut results = String::new();
    for guess_idx in 0..4 {
        let mut result = "⬜";
        for secret_idx in 0..4 {
            let num = guess[guess_idx];
            if num == modified_secret[secret_idx] {
                if guess_idx == secret_idx {
                    result = "🟩";
                    // Prevent future numbers from matching this value since 
                    // it has already been matched
                    modified_secret[guess_idx] = -1;
                    break;
                } else {
                    result = "🟨"
                }
            }
        } // secret_idx
        results += result;
        if guess_idx < 3 {
            results += " ";
        }
    } // guess_idx

    results
}

fn main() {
    // Generate 4 random numbers in range [0, 10)
    let mut secret: [i32; 4] = [0, 0, 0, 0];
    let mut rng = rand::thread_rng();
    for i in 0..4 {
        secret[i] = rng.gen_range(0..10);
    }

    // Read the user's input
    let stdin = std::io::stdin();
    let mut buf = String::new();
    println!("🟩 Green and Yellow 🟨");
    println!("Enter 4 numbers (separated by spaces) to make a guess.");
    loop {
        // Clear previous output, read in new input
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        
        // Convert string input to a vector of integers
        let guess_converted: Result<Vec<i32>, _> = buf.trim().split(' ').map(|s| s.parse()).collect();
        
        if guess_converted.is_err() {
            println!("Invalid input");
            continue;
        }

        let guess = guess_converted.unwrap();
        if guess.len() != 4 {
            println!("Must provide 4 numbers");
            continue;
        }

        // Print the results of the guess and secret matching
        let results = calc_green_and_yellow(&guess, &secret);
        println!("{:?}", results);
        
        if results == "🟩 🟩 🟩 🟩" {
            println!("Congratulations! You won!");
            break;
        }
    }
}

#[test]
fn test_green_and_yellow() {
    assert_eq!(calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 3, 4]), "🟩 🟩 🟩 🟩".to_string());
    assert_eq!(calc_green_and_yellow(&[1, 2, 3, 5], &[1, 2, 3, 4]), "🟩 🟩 🟩 ⬜".to_string());
    assert_eq!(calc_green_and_yellow(&[4, 3, 2, 1], &[1, 2, 3, 4]), "🟨 🟨 🟨 🟨".to_string());
    assert_eq!(calc_green_and_yellow(&[1, 2, 3, 1], &[1, 2, 3, 4]), "🟩 🟩 🟩 ⬜".to_string());
    assert_eq!(calc_green_and_yellow(&[1, 1, 1, 1], &[1, 2, 3, 4]), "🟩 ⬜ ⬜ ⬜".to_string());
    assert_eq!(calc_green_and_yellow(&[1, 2, 2, 2], &[2, 2, 2, 1]), "🟨 🟩 🟩 🟨".to_string());
    assert_eq!(calc_green_and_yellow(&[1, 3, 3, 2], &[2, 2, 2, 1]), "🟨 ⬜ ⬜ 🟨".to_string());
}
