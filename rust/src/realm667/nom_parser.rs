use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case, take_until},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace1, none_of},
    combinator::{map, map_res, opt, peek, recognize, value, eof},
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

/// Whitespace and comment parser (no newlines)
pub fn sp_no_newline(input: &str) -> IResult<&str, ()> {
    let line_comment = value((), pair(tag("//"), take_until("\n")));
    let block_comment = value((), delimited(tag("/*"), take_until("*/"), tag("*/")));
    let whitespace = value((), recognize(many1(alt((char(' '), char('\t'))))));
    
    map(many0(alt((whitespace, line_comment, block_comment))), |_| ())(input)
}

/// Wrapper to consume whitespace around a parser
pub fn ws<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(sp, inner, sp)
}

/// Wrapper to consume whitespace around a parser (no newlines)
pub fn ws_no_newline<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(sp_no_newline, inner, sp_no_newline)
}

/// Parse a GZValue or a complex expression (as a string)
pub fn parse_gz_value(input: &str) -> IResult<&str, GZValue> {
    // Peek to ensure we are not parsing a flag (+FLAG or -FLAG) as a value.
    // Negative numbers (-1, -0.5) are still allowed as values.
    if let Ok(_) = peek::<&str, _, nom::error::Error<&str>, _>(pair(alt((char('+'), char('-'))), alpha1))(input) {
         return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
    }

    ws(alt((
        parse_string_literal,
        // Match numbers ONLY if followed by a delimiter
        terminated(parse_float, peek(alt((
            value((), tag(",")),
            value((), tag(")")),
            value((), tag(";")),
            value((), tag("}")),
            value((), multispace1),
            value((), eof),
        )))),
        terminated(parse_integer, peek(alt((
            value((), tag(",")),
            value((), tag(")")),
            value((), tag(";")),
            value((), tag("}")),
            value((), multispace1),
            value((), eof),
        )))),
        parse_expression_value,
    )))(input)
}

pub fn parse_expression_value(input: &str) -> IResult<&str, GZValue> {
    let mut depth = 0;
    let mut end_pos = 0;
    let bytes = input.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'(' { depth += 1; }
        else if b == b')' {
            if depth == 0 { break; }
            depth -= 1;
        }
        else if (b == b',' || b == b';') && depth == 0 { break; }
        else if (b == b' ' || b == b'\t' || b == b'\n' || b == b'\r') && depth == 0 {
            if i > 0 { break; }
        }
        end_pos = i + 1;
    }
    if end_pos == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::AlphaNumeric)));
    }
    let val = &input[..end_pos];
    let rest = &input[end_pos..];
    let val_str = val.trim();
    
    // Try to parse as a function call if it contains parens
    if val_str.contains('(') {
        if let Ok((rem, call)) = parse_function_call(val_str) {
            if rem.trim().is_empty() {
                return Ok((rest, GZValue::FunctionCall(call)));
            }
        }
    }
    
    Ok((rest, GZValue::Identifier(val_str.to_string())))
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

/// Parse a function call
pub fn parse_function_call(input: &str) -> IResult<&str, GZFunctionCall> {
    let (input, name) = ws(recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_"), tag("."), tag("["), tag("]"))))
    )))(input)?;
    
    let (input, args) = delimited(
        ws(char('(')),
        separated_list0(ws(char(',')), parse_gz_value),
        ws(char(')'))
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
    
    let (input, frames) = ws_no_newline(recognize(many1(none_of(" \t\n\r{}/"))))(input)?;
    
    let (input, duration) = ws_no_newline(map_res(
        recognize(pair(opt(char('-')), digit1)),
        |s: &str| s.parse::<i32>()
    ))(input)?;
    
    let (input, modifiers) = many0(ws_no_newline(alt((
        tag_no_case("BRIGHT"),
        tag_no_case("NODELAY"),
        tag_no_case("SLOW"),
        tag_no_case("FAST"),
        tag_no_case("CANRAISE"),
    ))))(input)?;
    let is_bright = modifiers.iter().any(|m| m.to_lowercase() == "bright");
    
    // Try to parse actions, but don't fail if we can't find them
    let (input, actions) = match opt(alt((
        map(parse_function_call, |c| vec![c]),
        delimited(
            ws(char('{')),
            many0(terminated(ws(parse_function_call), opt(ws(char(';'))))),
            ws(char('}'))
        ),
        map(ws_no_newline(recognize(pair(alpha1, many0(alt((alphanumeric1, tag("_"))))))), |name| vec![GZFunctionCall {
            name: name.to_string(),
            args: vec![],
        }]),
    )))(input) {
        Ok((rem, Some(actions))) => (rem, actions),
        _ => (input, vec![]), // Fallback: no actions, but successfully parsed frame
    };

    // Optional semicolon
    let (input, _) = opt(ws_no_newline(char(';')))(input)?;

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
        if let Ok((next_input, label)) = terminated(ws(recognize(many1(alt((alphanumeric1, tag("_"), tag(".")))))), ws(char(':')))(input) {
            current_labels.push(label.to_string());
            input = next_input;
            continue;
        }

        // Try to parse flow control (Loop, Stop, Wait, Fail)
        let flow_control = alt::<&str, &str, nom::error::Error<&str>, _>((
            tag_no_case("loop"),
            tag_no_case("stop"),
            tag_no_case("wait"),
            tag_no_case("fail"),
        ));
        
        if let Ok((next_input, _)) = ws(flow_control)(input) {
            let (next_input, _) = opt(ws(char(';')))(next_input)?;
            input = next_input;
            // After flow control, clear labels as they are usually tied to what follows
            current_labels.clear();
            continue;
        }

        // Try to parse Goto separately for better error handling
        if let Ok((next_input, _)) = ws(tag_no_case("goto"))(input) {
            let (next_input, _label) = ws(recognize(many1(none_of(" \t\n\r;}"))))(next_input)?;
            let (next_input, _) = opt(ws(char(';')))(next_input)?;
            input = next_input;
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
                continue;
            }
            Err(_e) => {
                return Err(_e);
            }
        }
    }
    
    Ok((input, states))
}

/// Parse a property or flag
pub fn parse_property_or_flag(input: &str) -> IResult<&str, PropertyOrFlag> {
    let flag_parser = map(
        ws(recognize(pair(alt((char('+'), char('-'))), many1(alt((alphanumeric1, tag("_"))))))),
        |s: &str| PropertyOrFlag::Flag(s.to_string())
    );
    
    let prop_parser = map(
        pair(
            ws(recognize(pair(alt((alpha1, tag("_"))), many0(alt((alphanumeric1, tag("_"), tag("."))))))),
            terminated(separated_list0(ws(char(',')), parse_gz_value), opt(ws(char(';'))))
        ),
        |(name, values)| PropertyOrFlag::Property(name.to_string(), values)
    );

    alt((flag_parser, prop_parser))(input)
}

pub enum PropertyOrFlag {
    Property(String, Vec<GZValue>),
    Flag(String),
}

/// Parse an actor definition
pub fn parse_actor(input: &str) -> IResult<&str, ActorDefinition> {
    let (input, _) = ws(alt((tag_no_case("actor"), tag_no_case("class"))))(input)?;
    
    let (input, name) = ws(recognize(pair(alt((alpha1, tag("_"))), many0(alt((alphanumeric1, tag("_")))))))(input)?;
    
    let (input, parent) = opt(preceded(ws(char(':')), ws(recognize(pair(alt((alpha1, tag("_"))), many0(alt((alphanumeric1, tag("_")))))))))(input)?;
    
    let (input, ed_number) = opt(ws(map_res(digit1, |s: &str| s.parse::<i32>())))(input)?;
    
    let (input, _replaces) = opt(preceded(ws(tag_no_case("replaces")), ws(recognize(pair(alt((alpha1, tag("_"))), many0(alt((alphanumeric1, tag("_")))))))))(input)?;

    let (input, _) = ws(char('{'))(input)?;
    
    let mut actor = ActorDefinition {
        name: name.to_string(),
        parent: parent.map(|s| s.to_string()),
        ed_number,
        ..Default::default()
    };
    
    let mut input = input;
    loop {
        // Stop if we hit the end of the class
        if let Ok((next_input, _)) = ws(char('}'))(input) {
            input = next_input;
            break;
        }
        
        if input.is_empty() { break; }

        // Parse ZScript methods/functions
        if let Ok((next_input, (_modifier, _return_type, name, _))) = ws(tuple::<&str, _, nom::error::Error<&str>, _>((
            opt(alt((tag_no_case("action"), tag_no_case("override"), tag_no_case("virtual"), tag_no_case("static"), tag_no_case("protected"), tag_no_case("private"), tag_no_case("native")))),
            ws(recognize(pair(alt((alpha1, tag("_"))), many0(alt((alphanumeric1, tag("_"), tag("."))))))), // return type or void
            ws(recognize(pair(alt((alpha1, tag("_"))), many0(alt((alphanumeric1, tag("_"))))))), // name
            ws(delimited(char('('), take_until(")"), char(')'))),
            // No ws(char('{')) here yet
        )))(input) {
            if let Ok((next_input, _)) = ws(char::<&str, nom::error::Error<&str>>('{'))(next_input) {
                let mut temp_input = next_input;
                let mut body_actions = Vec::new();
                
                loop {
                    if let Ok((next, _)) = ws(char::<&str, nom::error::Error<&str>>('}'))(temp_input) {
                        temp_input = next;
                        break;
                    }
                    
                    if let Ok((next, action)) = terminated(ws(parse_function_call), opt(ws(char(';'))))(temp_input) {
                        body_actions.push(action);
                        temp_input = next;
                    } else {
                        return Err(nom::Err::Failure(nom::error::Error::new(temp_input, nom::error::ErrorKind::Tag)));
                    }
                }
                actor.methods.insert(name.to_string(), body_actions);
                input = temp_input;
                continue;
            }
        }

        // Handle ZScript Default block
        if input.trim_start().to_lowercase().starts_with("default") {
             let (next_input, _) = ws(tag_no_case("Default"))(input)?;
             let (next_input, _) = ws(char('{'))(next_input)?;
             let mut inner_input = next_input;
             loop {
                 if let Ok((final_input, _)) = ws(char('}'))(inner_input) {
                     inner_input = final_input;
                     break;
                 }
                 if inner_input.is_empty() { break; }

                 match parse_property_or_flag(inner_input) {
                     Ok((next_inner, item)) => {
                         apply_property_or_flag(&mut actor, item);
                         inner_input = next_inner;
                     }
                    Err(e) => {
                         return Err(e);
                     }
                 }
             }
             input = inner_input;
             continue;
        }

        // Handle States block
        if input.trim_start().to_lowercase().starts_with("states") {
            let (next_input, states) = parse_states_block(input)?;
            actor.states.extend(states);
            input = next_input;
            continue;
        }
        
        // Handle standalone property/flag
        if let Ok((next_input, item)) = parse_property_or_flag(input) {
            apply_property_or_flag(&mut actor, item);
            input = next_input;
            continue;
        }
        
        // If nothing matches, log and skip unexpected token instead of failing
        if let Ok((next_input, _)) = take_until::<&str, &str, nom::error::Error<&str>>("\n")(input) {
            println!("Warning: Skipping unexpected input");
            input = next_input;
            continue;
        } else {
            // If even taking until newline fails, just break/abort
            break;
        }
    }
    
    Ok((input, actor))
}

fn apply_property_or_flag(actor: &mut ActorDefinition, item: PropertyOrFlag) {
    match item {
        PropertyOrFlag::Flag(f) => {
            if f.starts_with('+') {
                actor.flags.push(f[1..].to_string());
            } else if f.starts_with('-') {
                // Ignore for now or handle removal
            }
        }
        PropertyOrFlag::Property(name, values) => {
            if values.is_empty() {
                actor.flags.push(name);
            } else if values.len() == 1 {
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

pub fn parse_sndinfo(input: &str) -> HashMap<String, String> {
    let mut sounds = HashMap::new();
    let mut input = input;

    while !input.trim().is_empty() {
        // Skip whitespace and comments
        if let Ok((next_input, _)) = sp(input) {
            input = next_input;
        }
        if input.trim().is_empty() { break; }

        let mut parse_entry = pair(
            ws(recognize(pair(alt((alpha1, tag("_"))), many0(alt((alphanumeric1, tag("_"), tag("."), tag("-"))))))),
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
                // If it starts with $, it might be a command. We skip it for now but properly.
                if input.starts_with('$') {
                    let mut temp_input = input;
                    while !temp_input.is_empty() && !temp_input.starts_with('\n') && !temp_input.starts_with('\r') {
                        temp_input = &temp_input[1..];
                    }
                    input = temp_input;
                    continue;
                }

                // If we don't recognize it, stop to avoid infinite loop or silent skip of errors
                break;
            }
        }
    }
    sounds
}
pub fn parse_document(input: &str) -> Result<Vec<ActorDefinition>, String> {
    let mut actors = Vec::new();
    let mut input = input;

    while !input.trim().is_empty() {
        // Consume whitespace and comments
        if let Ok((next_input, _)) = sp(input) {
            input = next_input;
        }

        if input.trim().is_empty() {
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
