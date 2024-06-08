use std::io;
// Text based game
// 3 paths that all lead to death
// - Forward path
// - Left path
// - Right path
// Each path should have a choice
// - The choice always leads to death.
// - A function for each path.
// - Lets try to do this mostly on our own, only using GPT when we are truly stuck.
// - Look back at our example code if needed, but try not to.
fn main() {
  let mut input: String = String::new();
  println!("Choose wisely of the path that will decide your destiny.");
  println!("Which path will you choose? 1, 2, or 3...");
  println!("1.) Foward");
  println!("2.) Left");
  println!("3.) Right");
  io::stdin().read_line(&mut input).expect("Failed to read the line!");

  loop {
    if input.trim() == "1" {
      let forward_input = forward_path(&mut input);
      if forward_input.trim() == "1" {
        println!("You held your ground, but sadly died to nothing");
        break;
      } else {
        println!("Running like a coward got you backstabbed");
      }
    } else if input.trim() == "2" {
      let left_input = left_path(&mut input);
      if left_input == "1" {
        println!("You get lost in the Temple of Water and drown...Should've equipped Zora's Tunic");
        break;
      } else {
        println!("Turns out mama raised a bitch");
        input.clear();
      }
    } else {
      let right_input = right_path(&mut input);
      if right_input.trim() == "1"{
        println!("A terrible life choice leads to death");
        break;
      } else {
        println!("I like where your heads at but unfortunately you still die.");
        break;
      }
    }
  }
  println!("All roads lead to death");
}

fn forward_path(input: &mut String) -> &mut String {
  println!("You chose the forward path!");
  println!("A sound rustles behind door 1...what will you do?");
  input.clear();

  println!("1.) Stand your ground");
  println!("2.) Run like a coward");
  io::stdin().read_line(input).expect("An issue occurred when reading the line");
  return input;
}

fn left_path(input: &mut String) -> &mut String {
  println!("You chose the left path!");
  println!("Now you stand in front of the Water Temple, what will you do?");
  input.clear();

  println!("1.) Traverse into the mist...");
  println!("2.) Run away like a little bitch.");
  io::stdin().read_line(input).expect("Provide a string representation of 1 or 2, bitch.");
  return input;
}

fn right_path(input: &mut String) -> &mut String {
  println!("You chose the right path!");
  println!("You find yourself in stuck in a cell with the infamous Tossed Salad Man. How do you handle this situation?");
  input.clear();

  println!("1.) Toss his salad");
  println!("2.) Stand up for yourself");
  io::stdin().read_line(input).expect("Incorrect value supplied");
  return input;
}
