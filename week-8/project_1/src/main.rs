use std::io;

fn main() {
    println!("Hello, please input your category:Intern, Administrator, Senior Administrator, Office Manager, Director, CEO, Research Assistant, PhD Candidate, Post-Doc Researcher, Senior Lecturer, Dean, Paralegal, Junior Associate, Associate, Senior Associate 1-2, Senior Associate 3-4, Partner, Placement, Classroom Teacher, Senior Teacher, Leading Teacher, Deputy Principal, Principal");

    let mut category_input = String::new();
    io::stdin().read_line(&mut category_input).expect("Failed to read your input!");

    let cleaned_input = category_input.trim().to_lowercase();

    let level = staff_level(&cleaned_input)
        .expect("ERROR: Job title not recognized in the public service table");

    println!("Validation successful! Your level is: {}", level);
}

fn staff_level(job_title: &str) -> Result<&'static str, &'static str> {
    let public_servant = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    if job_title == "intern" ||
       job_title == "paralegal" ||
       job_title == "placement" {
        return Ok(public_servant[0]);
    }

    if job_title == "administrator" ||
       job_title == "research assistant" ||
       job_title == "junior associate" ||
       job_title == "classroom teacher" {
        return Ok(public_servant[1]);
    }

    if job_title == "senior administrator" ||
       job_title == "phd candidate" ||
       job_title == "associate" ||
       job_title == "senior teacher" {
        return Ok(public_servant[2]);
    }

    if job_title == "office manager" ||
       job_title == "post-doc researcher" ||
       job_title == "senior associate 1-2" ||
       job_title == "leading teacher" {
        return Ok(public_servant[3]);
    }

    if job_title == "director" ||
       job_title == "senior lecturer" ||
       job_title == "senior associate 3-4" ||
       job_title == "deputy principal" {
        return Ok(public_servant[4]);
    }

    if job_title == "ceo" ||
       job_title == "dean" ||
       job_title == "partner" ||
       job_title == "principal" {
        return Ok(public_servant[5]);
    }

    Err("Job title not found!")
}