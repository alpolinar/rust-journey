const TWO: u32 = 1 + 1;

fn main() {
    println!("The value of TWO is: {TWO}");

    // declaration
    let x = 5;
    println!("The value of x is: {x}");
    // shadowing
    let x = x + 1;
    println!("The value of x is: {x}");

    {
        // inner context
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // Integer types
    // 8-bit    i8          u8
    // 16-bit   i16         u16
    // 32-bit   i32         u32
    // 64-bit   i64         u64
    // 128-bit  i128        u128
    // arch     isize       usize

    // Integer leterals
    let _decimal = 98_222;
    let _hex = 0xff;
    let _octal = 0o77;
    let _binary = 0b1111_0000;
    let _byte = b'A';

    // Floating types
    // f32 single-precision float
    // f64 double-precision float
    let _y: f64 = 2.0;
    let _z: f32 = 3.0;

    // Numeric operations
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3;
    let _remainder = 43 % 5;

    // Boolean types
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // Character type
    let _c = 'z';
    let _b: char = 'Z';
    let _heart_eyed_cat = '😻';

    // Compound type
    // Tuple
    let tup: (i32, f64, u8) = (-500, 6.4, 1);
    let (q, w, e) = tup;
    println!("The value of q: {q} w: {w} e: {e}");
    // accessing tuple values
    let val_1 = tup.0;
    let val_2 = tup.1;
    let val_3 = tup.2;
    println!("tup: ({val_1}, {val_2}, {val_3})");
    // mutate tuple
    let mut tups: (i32, i32) = (1, 2);
    tups.0 = 0;
    tups.1 += 5;

    // Array type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // initialize array to contain the same value for each element
    let _j = [3; 5];
    // accessing array elements
    let _first = a[0];
    let _second = a[1];
}
