// Fibonacci 數列
fn main() {
    let n = fibonacci(5);
    println!("{n}");
}

// 產生第 n 個 Fibonacci 數字
fn fibonacci(n: i32)-> i32 {
    if n<2 {
        return n;
    }

    let mut x = 1;
    let mut y = 1;
    let mut i = 2;
    while i < n {
        i += 1;
        
        (x, y) = (y, x+y);
        // println!("{i}: {y}");
    }
    return y;
}
