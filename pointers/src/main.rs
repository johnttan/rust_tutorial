fn main() {
    let x = &mut 5;

    if *x < 10 {
        let y = &x;

        println!("Oh no: {}", y);
        return;
    }

    *x -= 1;

    println!("Oh no: {}", x);
}
