fn main() {
  let x = 5;

  let space_ship: i32 = x + 1;

  {
    let x = space_ship.pow(2);
    println!("The value of x in the inner scope is: {x}");
  }

  println!("The value of x is: {}", space_ship);
}
