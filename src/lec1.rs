
pub fn test() {
    // sein x und y die seiten eines rechtwinkligen dreiecks, dann ist z die hypotenuse
    let x: f32 = 5.0;
    let y: f32 = 6.0;

    // z = wurzel aus x quadrat + y quadrat


    // Hier müsst ihr euren Code einfügen
    let z:f32 = (x*x + y*y).sqrt();
    
    println!("{0}, {1} => {2}", x, y, z);

}