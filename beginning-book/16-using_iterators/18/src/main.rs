fn main() {
    let slice1 = &[3, 4, 5];
    let slice2 = &[7, 8];
    let iterator = slice1.iter();
    for item_ref in iterator {
        print!("{} ", *item_ref);
    }
    print!("; ");
    iterator = slice2.iter();
    for item_ref in iterator {
        print!("{} ", *item_ref);
    }
}
