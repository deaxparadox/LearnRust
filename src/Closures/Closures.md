## Closures

Closure are inline anonymous function, such as **lambda** in python.

```rs
pub fn Closures() {
    let mut arr = [5,3,5,6,7,4,3];
    use std::cmp::Ordering;

    let  desc = |a: &i32, b: &i32| -> Ordering {
        if a < b { Ordering::Greater }
        else if a > b { Ordering::Less }
        else { Ordering::Equal }
    };

    arr.sort_by(desc);
    println!("{:?}", arr);


}
```

or directly specifing closures in inside **sort_by** 

```rs
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
```

## Examples

```rs
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
```