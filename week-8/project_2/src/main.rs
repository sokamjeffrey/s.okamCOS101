fn main() {
    // Create a vector of tuples holding (Candidate Name, Years of Experience)
    let candidates = vec![
        ("Chukwudi", 4),
        ("Amina", 7),
        ("Oluwaseun", 10),
        ("Fatima", 5),
        ("Emeka", 8),
    ];

    println!("--- EY Nigeria Developer Interview Evaluation ---");

    // Assume the first candidate has the highest experience initially
    let mut best_candidate = candidates[0];

    // Loop through the vector starting from the second element
    for candidate in &candidates {
        if candidate.1 > best_candidate.1 {
            best_candidate = *candidate; // Update if a higher experience is found
        }
    }

    // Display the winner
    println!("\nThe candidate with the highest years of experience is:");
    println!("Name: {}, Years of Experience: {} years", best_candidate.0, best_candidate.1);
}