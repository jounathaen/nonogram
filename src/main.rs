mod nono;


fn main() {
    let col : Vec<nono::Clues> = vec![vec![0], vec![1], vec![2]];
    let row : Vec<nono::Clues> = vec![vec![0], vec![2], vec![1]];
    let mut nono = nono::new (col, row,).unwrap();

    println!("{}", nono);
    nono.solve().unwrap();
    println!("{}", nono);
}
