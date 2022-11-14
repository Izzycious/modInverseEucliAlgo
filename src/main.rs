fn mod_inv(a: isize, module: isize) -> isize {
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
    println!("{}", mod_inv(29, 90)) 
  }