use std::vec;


pub fn test() {

    for i in 1..10 {
        println!("Zahl: {}", i);
    }

    let mut x:i32 = 0;
    while x < 14 {
        println!("Zahl2: {}", x);
        x = x + 1;
    }

    // Kleiner Gauss: Wir haben eingabe y; und wir wollen summe x = 1 + 2 + 3 + 4+ 5 + .. + y berechnen
    let y = 5;
    let mut erg = 0;
    for i in 0..y+1 {
        erg += i;
    } 
    println!("{}", erg);

    let mut meineliste:Vec<i32> = vec![1, 2, 3, 5, 6, 7];
    println!("{}", meineliste[4]);
    for k in &meineliste {
        println!("{}", k)
    }
    // denkte daran immer & zu benutzen wenn ihr über eine Liste iteriert, sonst könnt ihr die Liste nach der for schleife nicht mehr
    // ändern


    
    // Hausaufgabe: Summenprodukt;
    // Gegeben ist eine Matrix -> Eine List von Listen / 2d Liste oder 2D Vector
    // 1 4 5 7
    // 2 5 6 7
    // 1 7 3 0
    // Wir suchen dass Summenprodukt x 
    // ist so definiert am Beispiel: (1 * 2 * 1) + (4 * 5 * 7) + ..
    let matrix:Vec<Vec<i32>> = vec![
        vec![1, 4, 5, 7],
        vec![2, 5, 6, 7],
        vec![1, 7, 3, 0],
    ];

    let mut summe = 0;
    for zeile in 0..matrix[0].len() {
        let mut produkt = 1;
        for liste in &matrix {
            produkt *= liste[zeile];
        }
        summe += produkt;
    }
    println!("Summenprodukt: {}", summe);

    





}