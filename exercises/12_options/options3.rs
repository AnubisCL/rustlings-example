#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    // match &optional_point { // 匹配整个 Option 的引用
    match optional_point {
        // 这是模式匹配语法，表示"匹配一个引用并将其绑定到 p
        Some(ref p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}
