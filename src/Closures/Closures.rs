pub fn Closures() {
    let mut arr = [5,3,5,6,7,4,3];
    use std::cmp::Ordering;

    arr.sort_by(|a, b| {
        if a < b { Ordering::Greater }
        else if a > b { Ordering::Less }
        else { Ordering::Equal }
    });
    println!("{:?}", arr);

}

pub fn Examples() {
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];

    // Descending order
    arr.sort_by(|a, b| b.cmp(a));
    println!("{:?}", arr);

    // Inversion
    arr.sort_by(|a, b| (&-*b).cmp(&-*a));
    println!("{:?}", arr);

    // Ascending order
    arr.sort_by(|a, b| a.cmp(b));
    println!("{:?}", arr);

    // Ascending order
    arr.sort();
    println!("{:?}", arr);
}