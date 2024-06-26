// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//




#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
     
    
    *z += 1000;
    assert_eq!(x, 1200);
}
/*The key changes:

Declare x
Declare borrow of y from x
Use y
Declare borrow of z from x (no conflict since y is no longer borrowed)
Use z
Assertion
This works because:

y borrows x
y is used
y's borrow ends
z can now borrow x since it is not currently borrowed
By reordering so the borrows do not overlap in time, it compiles without issues. */