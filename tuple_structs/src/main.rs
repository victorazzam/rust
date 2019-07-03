fn main() {
    struct Colour(i32, i32, i32);
    struct Point(i32, i32, i32);

    // These vars are different types since they use different tuple structs
    // If a function accepts Colour, it cannot accept Point in its place
    let red = Colour(255, 0, 0);
    let origin = Point(0, 0, 0);
}
