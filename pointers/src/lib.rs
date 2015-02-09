#[test]
fn it_works() {
  let x = &mut 5;
  if *x < 10 {
    let y = &x;

    println!("Oh no: {}", y );
  }

  *x -= 1;

  println!("yea x {}", x);
}
