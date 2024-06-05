// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

// suppression du ";" a la fin de "num * num" , pour que la fonction nous retourne la valeur souhaiter.
// Un ";" indique la fin d'une instruction qui est "num * num" ne retournerais rien.

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
