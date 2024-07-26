fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    let (name,age) = cat;  // 元组的使用

    println!("{name} is {age} years old");
}
