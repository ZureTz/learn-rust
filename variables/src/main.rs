fn main() {
  let a: [i32; 5] = [1, 2, 3, 4, 5];

  println!("Please enter an array index.");

  let mut index = String::new();

  std::io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

  let index: usize = index
    .trim()
    .parse::<usize>()
    .expect("Please enter a number.");

  println!("The value at index {} is: {}", index, a[index]);
}
