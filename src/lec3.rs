use std::vec;


pub fn addiere(a:&mut f32, b:f32) -> f32 {
    *a = *a + *a;
    return *a + b;
}


// Schreibe die Fibonacci funktion
// Mit if else und so weiter 
/// f(0) = 1; f(1) = 1
/// f(n) = f(n-1) + f(n-2);


// Schreibe ein Funktion, welche einen Vektor als eingabe bekommt sowie eine mutierbare referenz summe
// speichere die summe zahlen im Vektor in der summenvariable ab


pub fn test() {

    let mut x:f32 = 0.6;
    let y:f32 = 0.7;

    let z:f32 = addiere(&mut x, y);

    println!("{}", z);


    println!("{}", x);
    




}