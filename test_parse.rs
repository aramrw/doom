use nom::{bytes::complete::is_not, error::Error, IResult};

fn test() {
    let input = "Rock1:\n\t\tDROK A -1\n";
    let (input, _) = is_not::<&str, &str, Error<&str>>(" \t\n\r;}")(input).unwrap();
    println!("Remaining: {:?}", input);
}

fn main() {
    test();
}
