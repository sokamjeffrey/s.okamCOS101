use std::io;

fn main() {
    // Storing professions and their respective level brackets using vectors
    // Let's use a vector of tuples: (Profession, Min_Years, Max_Years, APS_Level)
    let aps_table = vec![
        ("Lawyer", 0, 2, "APS 1-2 (Paralegal / Placement)"),
        ("Lawyer", 3, 5, "APS 3-5 (Paralegal / Junior Associate)"),
        ("Lawyer", 6, 8, "APS 5-8 (Junior Associate / Senior Associate)"),
        ("Lawyer", 9, 10, "EL1 8-10 (Senior Associate 1-2)"),
        ("Lawyer", 11, 15, "EL2 10-13 (Senior Associate 3-4)"),
        ("Lawyer", 16, 30, "SES (Partner)"),
        
        // You can add other professions like Teacher, Academic, or Office Admin similarly:
        ("Teacher", 0, 2, "APS 1-2 (Placement)"),
        ("Teacher", 3, 5, "APS 3-5 (Classroom Teacher)"),
        ("Teacher", 6, 8, "APS 5-8 (Senior Teacher)"),
        ("Teacher", 9, 10, "EL1 8-10 (Leading Teacher)"),
        ("Teacher", 11, 30, "SES (Principal)"),
    ];

    let mut input_prof = String::new();
    let mut input_years = String::new();

    println!("--- Federal Service APS Level Checker ---");
    println!("Enter profession (e.g., Lawyer, Teacher):");
    io::stdin().read_line(&mut input_prof).expect("Failed to read input");
    let profession = input_prof.trim();

    println!("Enter years of experience:");
    io::stdin().read_line(&mut input_years).expect("Failed to read input");
    let years: i32 = input_years.trim().parse().expect("Please enter a valid number");

    let mut found = false;

    // Iterate through the vector to find the matching bracket
    for entry in &aps_table {
        if entry.0.eq_ignore_ascii_case(profession) && years >= entry.1 && years <= entry.2 {
            println!("\n[Match Found] Profession: {}, Years: {} -> Level: {}", entry.0, years, entry.3);
            found = true;
            break;
        }
    }

}