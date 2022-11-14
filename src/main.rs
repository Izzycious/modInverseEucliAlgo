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
    let mut ab = (module, a);
    let mut xy = (0, 1);

    
   
    while ab.1 != 0 {
      xy = (xy.1, xy.0 - (ab.0/ab.1) * xy.1);
      ab = (ab.1, ab.0 % ab.1);
    }

    while xy.0 < 0 {
      xy.0 += module;
    }
    xy.0
  }

  fn main() {
    println!("{}", mod_inverse(29, 90)) 
  }