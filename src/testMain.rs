use crate::SVM::Svm;

fn main() {
    // Das Makro `println!` gibt Text auf der Konsole aus.
    // Das Ausrufezeichen (!) zeigt an, dass es sich um ein Makro handelt und nicht um eine normale Funktion.
    let test = Svm::new(3, None);
    println!("Hallo Welt!");
}