
use std::io;

fn main() {
    // APS levels (index-aligned from 0..5)
    let aps_levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic      = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer        = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher       = vec!["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    
    println!("\nAPS Table");
    println!("{:<12} | {:<20} | {:<22} | {:<23} | {:<20}",
        "Level", "Office Administrator", "Academic", "Lawyer", "Teacher");
    println!("{}", "-".repeat(110));
    for i in 0..aps_levels.len() {
        println!("{:<12} | {:<20} | {:<22} | {:<23} | {:<20}",
            aps_levels[i],
            office_admin[i],
            academic[i],
            lawyer[i],
            teacher[i]
        );
    }

    
    println!("\nEnter profession (Office Administrator / Academic / Lawyer / Teacher):");
    let mut profession_in = String::new();
    io::stdin().read_line(&mut profession_in).expect("Failed to read input");
    let profession = profession_in.trim().to_lowercase();

    
    let (roles_vec, profession_label) = match profession.as_str() {
        "office administrator" => (&office_admin, "Office Administrator"),
        "academic"             => (&academic, "Academic"),
        "lawyer"               => (&lawyer, "Lawyer"),
        "teacher"              => (&teacher, "Teacher"),
        _ => {
            println!("\nUnknown profession: '{}'", profession_in.trim());
            println!("Please choose exactly one of: Office Administrator, Academic, Lawyer, Teacher.");
            return;
        }
    };


    println!("\nAvailable roles under '{}':", profession_label);
    for i in 0..aps_levels.len() {
        println!("  {} → {}", aps_levels[i], roles_vec[i]);
    }

    
    println!("\nEnter role shown above:");
    let mut role_in = String::new();
    io::stdin().read_line(&mut role_in).expect("Failed to read input");
    let role_entered = role_in.trim();

    let mut found = false;
    for i in 0..aps_levels.len() {
        if roles_vec[i].eq(role_entered) {
            println!("\n{} ({}) is in {}.", roles_vec[i], profession_label, aps_levels[i]);
            found = true;
            break;
        }
    }
    if !found {
        println!("\nRole '{}' not found under '{}'. Please pick one from the list above.", role_entered, profession_label);
    }

    
    
}
