fn main() {
    // These live on the STACK:
    let x = 42;
    let y = 3.14;
    let z = true;
    let arr = [1, 2, 3];          // fixed size: 3 * 4 = 12 bytes
    let big_boy = [1_000_000];

    // These have heap components:
    let s1 = String::from("hello");            // text on heap
    let s2 = "world";                          // &str — NOT on heap (in the binary)
    let v = vec![1, 2, 3, 4, 5];              // Vec — elements on heap
    let vb = vec![1*1_000_001];
    let long_str = String::from("this is a verry long text to see what the size could be");

    print_info("x (i32)", std::mem::size_of_val(&x));
    print_info("y (f64)", std::mem::size_of_val(&y));
    print_info("arr [i32; 3]", std::mem::size_of_val(&arr));
    print_info("big boy", std::mem::size_of_val(&big_boy));
    print_info("String", std::mem::size_of_val(&s1));
    print_info("&str", std::mem::size_of_val(&s2));
    print_info("Vec<i32>", std::mem::size_of_val(&v));
    print_info("Vec<usize>", std::mem::size_of_val(&vb));
    print_info("long string", std::mem::size_of_val(&long_str));
}

fn print_info(name: &str, size: usize) {
    println!("{name}: {size} bytes on the stack");
}
