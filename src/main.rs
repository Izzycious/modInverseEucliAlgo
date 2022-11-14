/*
Declaring a function that **modInverse** that takes in a 
 (a and module) of 
 isize(which is either 32/64bits) depending on the sytem.

 In rust **isize** differentiate from **usize** such that
 isize:  can be negative, and is generally used for offsets to addresses, positions, indices, or lengths.
         and in the code below we're dealing with index numbers
 usize:  cannot be negative and is generally used for memory addresses, positions, indices, lengths (or sizes!).
*/ 

fn mod_inverse(a: isize, module: isize) -> isize {
  /*
  Declaring a mutable variable in rust 
  ab, xy 
  This is to note that the values of the will be changed at the continuation of the program while executing
  */
    let mut ab = (module, a);
    let mut xy = (0, 1);

  /*Passing a condition using while loop
    the use of the dot notation is an auto-referencing coercion of types.

    xy = 
  */
   
    while ab.1 != 0 {
      xy = (xy.1, xy.0 - (ab.0/ab.1) * xy.1);
      ab = (ab.1, ab.0 % ab.1);
    }

    //implementation of euclidean Algorithm

    while xy.0 < 0 {
      xy.0 += module;
    }
    xy.0
  }

  fn main() {
    println!("{}", mod_inverse(482, 790)) // print the mod inverse of the value passed which is (482,790)
  }