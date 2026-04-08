use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case, take_until},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace1, none_of},
    combinator::{map, map_res, opt, recognize, value},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use std::collections::HashMap;
use crate::realm667::actor::{ActorDefinition, GZValue, GZFunctionCall, StateFrame};

/// Whitespace and comment parser
pub fn sp(input: &str) -> IResult<&str, ()> {
    let line_comment = value((), pair(tag("//"), take_until("\n")));
    let block_comment = value((), delimited(tag("/*"), take_until("*/"), tag("*/")));
    let whitespace = value((), multispace1);
    
    map(many0(alt((whitespace, line_comment, block_comment))), |_| ())(input)
}

/// Wrapper to consume whitespace around a parser
pub fn ws<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(sp, inner, sp)
}

/// Parse a GZValue
pub fn parse_gz_value(input: &str) -> IResult<&str, GZValue> {
    alt((
        parse_float,
        parse_integer,
        parse_string_literal,
        parse_identifier_value,
    ))(input)
}

fn parse_integer(input: &str) -> IResult<&str, GZValue> {
    map_res(
        recognize(pair(opt(char('-')), digit1)),
        |s: &str| s.parse::<i32>().map(GZValue::Integer)
    )(input)
}

fn parse_float(input: &str) -> IResult<&str, GZValue> {
    map_res(
        recognize(tuple((opt(char('-')), digit1, char('.'), digit1))),
        |s: &str| s.parse::<f64>().map(GZValue::Float)
    )(input)
}

fn parse_string_literal(input: &str) -> IResult<&str, GZValue> {
    map(
        delimited(char('"'), is_not("\""), char('"')),
        |s: &str| GZValue::String(s.to_string())
    )(input)
}

fn parse_identifier_value(input: &str) -> IResult<&str, GZValue> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"), tag("+"), tag("-"))),
            many0(alt((alphanumeric1, tag("_"), tag("."), tag("-"), tag("+"), tag("$"))))
        )),
        |s: &str| GZValue::Identifier(s.to_string())
    )(input)
}

/// Parse a function call
pub fn parse_function_call(input: &str) -> IResult<&str, GZFunctionCall> {
    let (input, name) = ws(recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_"), tag("."))))
    )))(input)?;
    
    let (input, args) = delimited(
        char('('),
        separated_list0(ws(char(',')), parse_gz_value),
        char(')')
    )(input)?;
    
    Ok((input, GZFunctionCall {
        name: name.to_string(),
        args,
    }))
}

/// Parse a state frame
pub fn parse_state_frame(input: &str) -> IResult<&str, StateFrame> {
    let (input, sprite_prefix) = ws(recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_"))))
    )))(input)?;
    
    let (input, frames) = ws(recognize(many1(none_of(" \t\n\r{}/"))))(input)?;
    
    let (input, duration) = ws(map_res(
        recognize(pair(opt(char('-')), digit1)),
        |s: &str| s.parse::<i32>()
    ))(input)?;
    
    let (input, modifier) = opt(ws(tag_no_case("BRIGHT")))(input)?;
    let is_bright = modifier.is_some();
    
    // Actions can be a single function call, a block of them, or nothing
    let (input, actions) = alt((
        map(parse_function_call, |c| vec![c]),
        delimited(
            ws(char('{')),
            many0(terminated(ws(parse_function_call), opt(ws(char(';'))))),
            ws(char('}'))
        ),
        // Fallback for simple identifier actions without parens
        map(ws(recognize(pair(alpha1, many0(alt((alphanumeric1, tag("_"))))))), |name| vec![GZFunctionCall {
            name: name.to_string(),
            args: vec![],
        }]),
        map(sp, |_| vec![]),
    ))(input)?;

    // Optional semicolon
    let (input, _) = opt(ws(char(';')))(input)?;

    Ok((input, StateFrame {
        sprite_prefix: sprite_prefix.to_string(),
        frames: frames.to_string(),
        duration,
        actions,
        is_bright,
    }))
}

/// Parse a states block
pub fn parse_states_block(input: &str) -> IResult<&str, HashMap<String, Vec<StateFrame>>> {
    let (input, _) = ws(tag_no_case("states"))(input)?;
    let (input, has_braces) = map(opt(ws(char('{'))), |o| o.is_some())(input)?;
    
    let mut states = HashMap::new();
    let mut current_labels = Vec::new();
    let mut input = input;
    
    loop {
        // Check for end of block
        if has_braces {
            if let Ok((next_input, _)) = ws(char('}'))(input) {
                input = next_input;
                break;
            }
        } else {
            // If no braces, we stop if we see something that looks like an actor-level keyword or closing brace of actor
            if input.trim().is_empty() || input.trim().starts_with('}') {
                break;
            }
            // Also stop if we see "Default" or another "States" or "Actor/Class"
            let lower_input = input.to_lowercase();
            if lower_input.trim_start().starts_with("default") || 
               lower_input.trim_start().starts_with("states") ||
               lower_input.trim_start().starts_with("actor") ||
               lower_input.trim_start().starts_with("class") {
                break;
            }
        }
        
        // Try to parse a label (e.g., "Spawn:")
        if let Ok((next_input, label)) = terminated(ws(alphanumeric1), ws(char(':')))(input) {
            current_labels.push(label.to_string());
            input = next_input;
            continue;
        }
        
        // Try to parse flow control (Loop, Stop, Wait, Goto)
        let flow_control = alt((
            tag_no_case("loop"),
            tag_no_case("stop"),
            tag_no_case("wait"),
            tag_no_case("fail"),
            recognize(pair(tag_no_case("goto"), preceded(multispace1, alphanumeric1))),
        ));
        
        if let Ok((next_input, _)) = ws(flow_control)(input) {
            let (next_input, _) = opt(ws(char(';')))(next_input)?;
            input = next_input;
            // After flow control, clear labels as they are usually tied to what follows
            current_labels.clear();
            continue;
        }
        
        // Try to parse a state frame
        match parse_state_frame(input) {
            Ok((next_input, frame)) => {
                for label in &current_labels {
                    states.entry(label.clone()).or_insert_with(Vec::new).push(frame.clone());
                }
                input = next_input;
            }
            Err(e) => return Err(e),
        }
    }
    
    Ok((input, states))
}

/// Parse a property or flag
pub fn parse_property_or_flag(input: &str) -> IResult<&str, PropertyOrFlag> {
    alt((
        // Flag: +SOLID or -SOLID
        map(
            recognize(pair(alt((char('+'), char('-'))), alphanumeric1)),
            |s: &str| PropertyOrFlag::Flag(s.to_string())
        ),
        // Property: Health 100
        map(
            pair(
                ws(recognize(pair(alpha1, many0(alt((alphanumeric1, tag("."))))))),
                terminated(separated_list0(ws(char(',')), parse_gz_value), opt(ws(char(';'))))
            ),
            |(name, values)| PropertyOrFlag::Property(name.to_string(), values)
        )
    ))(input)
}

pub enum PropertyOrFlag {
    Property(String, Vec<GZValue>),
    Flag(String),
}

/// Parse an actor definition
pub fn parse_actor(input: &str) -> IResult<&str, ActorDefinition> {
    let (input, _) = ws(alt((tag_no_case("actor"), tag_no_case("class"))))(input)?;
    
    let (input, name) = ws(recognize(pair(alpha1, many0(alt((alphanumeric1, tag("_")))))))(input)?;
    
    let (input, parent) = opt(preceded(ws(char(':')), ws(recognize(pair(alpha1, many0(alt((alphanumeric1, tag("_")))))))))(input)?;
    
    let (input, ed_number) = opt(ws(map_res(digit1, |s: &str| s.parse::<i32>())))(input)?;
    
    let (input, _replaces) = opt(preceded(ws(tag_no_case("replaces")), ws(recognize(pair(alpha1, many0(alt((alphanumeric1, tag("_")))))))))(input)?;

    let (input, _) = ws(char('{'))(input)?;
    
    let mut actor = ActorDefinition {
        name: name.to_string(),
        parent: parent.map(|s| s.to_string()),
        ed_number,
        ..Default::default()
    };
    
    let mut input = input;
    loop {
        if let Ok((next_input, _)) = ws(char('}'))(input) {
            input = next_input;
            break;
        }
        
        // Handle ZScript Default block
        if let Ok((next_input, _)) = ws(tag_no_case("Default"))(input) {
             let (next_input, _) = ws(char('{'))(next_input)?;
             let mut inner_input = next_input;
             loop {
                 if let Ok((final_input, _)) = ws(char('}'))(inner_input) {
                     inner_input = final_input;
                     break;
                 }
                 match parse_property_or_flag(inner_input) {
                     Ok((next_inner, item)) => {
                         apply_property_or_flag(&mut actor, item);
                         inner_input = next_inner;
                     }
                     Err(e) => return Err(e),
                 }
             }
             input = inner_input;
             continue;
        }

        // Handle States block
        if let Ok((next_input, states)) = parse_states_block(input) {
            actor.states.extend(states);
            input = next_input;
            continue;
        }
        
        // Handle standalone property/flag
        match parse_property_or_flag(input) {
            Ok((next_input, item)) => {
                apply_property_or_flag(&mut actor, item);
                input = next_input;
            }
            Err(_e) => {
                // Skip unknown things in actor block for robustness
                if let Ok((next_input, _)) = ws(char('{'))(input) {
                    // Skip balanced block
                    let mut depth = 1;
                    let mut temp_input = next_input;
                    while depth > 0 && !temp_input.is_empty() {
                        if let Ok((next, _)) = char::<&str, nom::error::Error<&str>>('{')(temp_input) {
                            depth += 1;
                            temp_input = next;
                        } else if let Ok((next, _)) = char::<&str, nom::error::Error<&str>>('}')(temp_input) {
                            depth -= 1;
                            temp_input = next;
                        } else if let Ok((next, _)) = multispace1::<&str, nom::error::Error<&str>>(temp_input) {
                            temp_input = next;
                        } else if !temp_input.is_empty() {
                            temp_input = &temp_input[1..];
                        }
                    }
                    input = temp_input;
                } else if let Ok((next_input, _)) = ws(recognize(many1(none_of(" \t\n\r{}"))))(input) {
                    input = next_input;
                } else if !input.is_empty() {
                    input = &input[1..];
                } else {
                    break;
                }
            }
        }
    }
    
    Ok((input, actor))
}

fn apply_property_or_flag(actor: &mut ActorDefinition, item: PropertyOrFlag) {
    match item {
        PropertyOrFlag::Flag(f) => {
            if f.starts_with('+') {
                actor.flags.push(f[1..].to_string());
            }
        }
        PropertyOrFlag::Property(name, values) => {
            if !values.is_empty() {
                if values.len() == 1 {
                    actor.properties.insert(name, values[0].clone());
                } else {
                    let joined = values.iter()
                        .map(|v| v.to_string_lossy())
                        .collect::<Vec<_>>()
                        .join(", ");
                    actor.properties.insert(name, GZValue::String(joined));
                }
            }
        }
    }
}

pub fn parse_sndinfo(input: &str) -> HashMap<String, String> {
    let mut sounds = HashMap::new();
    let mut input = input;
    
    while !input.trim().is_empty() {
        let mut parse_entry = pair(
            ws(recognize(pair(alpha1, many0(alt((alphanumeric1, tag("_"), tag("."), tag("-"))))))),
            alt((
                map(ws(delimited(char('"'), is_not("\""), char('"'))), |s: &str| s.to_string()),
                map(ws(recognize(many1(none_of(" \t\n\r")))), |s: &str| s.to_string())
            ))
        );
        
        match parse_entry(input) {
            Ok((next_input, (alias, path))) => {
                sounds.insert(alias.to_uppercase(), path);
                input = next_input;
            }
            Err(_) => {
                // Skip one token or whitespace
                if let Ok((next, _)) = multispace1::<&str, nom::error::Error<&str>>(input) {
                    input = next;
                } else if !input.is_empty() {
                    let mut next_token = recognize::<&str, _, nom::error::Error<&str>, _>(many1(none_of(" \t\n\r")));
                    if let Ok((next, _)) = next_token(input) {
                        input = next;
                    } else {
                        input = &input[1..];
                    }
                } else {
                    break;
                }
            }
        }
    }
    sounds
}

pub fn parse_document(input: &str) -> Result<Vec<ActorDefinition>, String> {
    let mut actors = Vec::new();
    let mut input = input;
    
    while !input.trim().is_empty() {
        // Try to find the next actor or class
        let mut actor_start = alt::<&str, &str, nom::error::Error<&str>, _>((tag_no_case("actor"), tag_no_case("class")));
        
        // Skip everything until "actor" or "class"
        let mut temp_input = input;
        loop {
            if temp_input.is_empty() {
                input = "";
                break;
            }
            if let Ok(_) = actor_start(temp_input) {
                input = temp_input;
                break;
            }
            // Skip one char or whitespace
            if let Ok((next, _)) = multispace1::<&str, nom::error::Error<&str>>(temp_input) {
                temp_input = next;
            } else if !temp_input.is_empty() {
                temp_input = &temp_input[1..];
            } else {
                input = "";
                break;
            }
        }
        
        if input.is_empty() {
            break;
        }

        match parse_actor(input) {
            Ok((next_input, actor)) => {
                actors.push(actor);
                input = next_input;
            }
            Err(e) => {
                return Err(format!("Parse error: {:?}", e));
            }
        }
    }
    
    Ok(actors)
}
