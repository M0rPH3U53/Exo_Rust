// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

// ajout du chiffre 4 a la fin du else car il y a 4 type.

pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2.0
    } else if animal == "snake" {
        3
    } else {
        4
    };
