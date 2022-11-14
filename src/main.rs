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

  and also stands for the intergers to be passed in calculating the multiplicative inverse using euclidean algorithm
  */
    let mut ab = (module, a);
    let mut xy = (0, 1);

  /*Passing a condition using while loop
    the use of the dot notation is an auto-referencing coercion of types.

    choosing the while loop is for us to repeatedly execute the code of block statement
    such that we can fulfil result of 1 while getting our inverser
  */
   // ab.1 must not be zero
    while ab.1 != 0 {
      /*
      Shows that the multiplicative inverse in euclidean algorithm
        1 ≡ by mod a which can also be denoted as R = a - bq
        * where R is the reminder
          a is an integer
          b is an integer
          q is a quotient
        
        the value xy.1 xy.0 subtracting the divisor of (ab.0/ab.1) which is the modulus
        multiplying to the given quotient
        xy.1 is the quotient
        xy.0 is the first integer
      */
      xy = (xy.1, xy.0 - (ab.0/ab.1) * xy.1);
      // This ensures that the modulo multiplicative inverse is always within the modulo
      ab = (ab.1, ab.0 % ab.1);
    }

    //implementation of euclidean Algorithm
    /*
    To pass this such that xy.0 is lesser than 0
    then our xy.0 is plus or equal to the modulus gotten
    */
    while xy.0 < 0 {
      xy.0 += module;
    }
    xy.0 // here we return the xy.0
  }

  fn main() {
    println!("{}", mod_inverse(482, 790)) // print the mod inverse of the value passed which is (482,790)
  }