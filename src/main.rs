static mut WEIGHT: i32 = 0;

fn init_weight() {
    unsafe{ WEIGHT = 0; }
}

fn add_weight(w: i32) {
    unsafe{ WEIGHT += w; }
}

fn weight() -> i32 {
    unsafe{ WEIGHT }
}

fn gcd(mut m: i32, mut n: i32) -> i32 {
    let mut r = 0;
    add_weight(10);
    if n > m {
        add_weight(8);
        r = m;
        add_weight(8);
        m = n;
        add_weight(8);
        n = r;
        add_weight(8);
    } else {
        add_weight(2);
    }
    r = m % n;
    add_weight(10);
    while r != 0 {
        add_weight(8);
        m = n;
        add_weight(8);
        n = r;
        add_weight(8);
        r = m % n;
        add_weight(8);
    }
    add_weight(2);
    add_weight(10);
    add_weight(10);
    n
}

fn main() {
    gcd(1, 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        init_weight();
        println!("{}, W = {}", gcd(15, 4), weight());
        init_weight();
        println!("{}, W = {}", gcd(6, 5), weight());
        init_weight();
        println!("{}, W = {}", gcd(6, 2), weight());
        init_weight();
        println!("{}, W = {}", gcd(4, 12), weight());
        
    }
}

