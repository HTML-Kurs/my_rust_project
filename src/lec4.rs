use std::vec;

// Ende ist exclusive
pub fn bin_search(liste:&Vec<i32>, gesucht:i32, anfang:usize, ende:usize) -> usize {
    if ende <= anfang {
        return liste.len();
    }
    let mitte = anfang + (ende - anfang) / 2;

    if liste[mitte] == gesucht {
        return mitte;
    } else if liste[mitte] < gesucht {
        return bin_search(liste, gesucht, mitte, ende);
    } else {
        return bin_search(liste, gesucht, anfang, mitte);
    }
}


pub fn test() {

    let myList = vec![1, 3, 5, 7, 12, 42, 53, 1231, 123123, 1233211, 4449494];

    // Gegeben Element x; Welchen Index hat x in myList?

    println!("{}", bin_search(&myList, 12, 0 , myList.len()));





}