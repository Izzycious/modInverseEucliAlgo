fn mod_in(a: isize, module: isize) -> isize {
    let mut ab = (module, a);
    let mut xy = (0,1);

    while ab != 0 {
        xy = (xy.1, xy.0 - (ab.0/mn.1) xy.1);
        ab = (ab.1, ab.0 % ab.1);
    }
}