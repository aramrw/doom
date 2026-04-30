use nom::{bytes::complete::{is_not, tag}, error::Error, IResult, sequence::delimited, character::complete::{char, space1}};

fn main() {
    let input = "goto Rock1\n";
    // Simulate ws(tag_no_case("goto"))
    let (input, _) = nom::sequence::delimited(
        nom::character::complete::multispace0,
        tag::<&str, &str, Error<&str>>("goto"),
        nom::character::complete::multispace0
    )(input).unwrap();
    
    // Now call is_not(" \t\n\r;}")
    // This is basically "take all characters until a forbidden character"
    
    let result: IResult<&str, &str, Error<&str>> = is_not(" \t\n\r;}")(input);
    match result {
        Ok((rest, val)) => println!("Result: Ok({:?}, {:?})", rest, val),
        Err(e) => println!("Error: {:?}", e),
    }

    // Now try it with leading whitespace:
    let input2 = " Rock1\n";
    let result2: IResult<&str, &str, Error<&str>> = is_not(" \t\n\r;}")(input2);
    match result2 {
        Ok((rest, val)) => println!("Result2: Ok({:?}, {:?})", rest, val),
        Err(e) => println!("Error2: {:?}", e),
    }
}
