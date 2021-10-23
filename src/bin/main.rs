use vec2d::*;

fn main() {
    let mut v = matrix![[1, 2, 3], [4, 5, 6]];
    //*v[0] = [5, 6];
    //println!("v: {}, v.as_ptr(): {}", &v.v as *const )
    println!("[[{}, {}], [{}, {}]]", v[0][0], v[0][1], v[1][0], v[1][1]);
}