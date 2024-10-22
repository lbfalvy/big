mod big;

fn big_dob() -> &'static [u8] {
    for i in 0..20 {
        use big::BIG_DOB;
        if i == 19 {
            return BIG_DOB
        }
    }
    panic!()
}

fn main() {
    print!("{:?}", big_dob());
}
