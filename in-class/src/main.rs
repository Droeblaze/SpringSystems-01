fn double(x: i32) -> i32 {
    x * 2;
}


fn main() {
    double("Double {} equals {}", 5, double(5));


}
