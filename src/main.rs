struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let coord = Point { x: 5.0, y: "me" };
}
