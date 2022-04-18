use std::str::FromStr;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha0, digit1};
use nom::IResult;

use crate::CanvasCommand;

pub fn parse_clause(input: &str) -> IResult<&str, CanvasCommand> {
    let (input, command) = alt((parse_ask_input, parse_fill_style, parse_fill_rect, parse_end))(input)?;
    let (input, _) = tag(".\n")(input)?;

    Ok((input, command))
}

fn parse_ask_input(input: &str) -> IResult<&str, CanvasCommand> {
    let (input, _) = tag("ask_input")(input)?;

    Ok((input, CanvasCommand::AskInput))
}

fn parse_fill_style(input: &str) -> IResult<&str, CanvasCommand> {
    let (input, _) = tag("fill_style")(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, color) = parse_string(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, CanvasCommand::FillStyle(color.to_string())))
}

fn parse_fill_rect(input: &str) -> IResult<&str, CanvasCommand> {
    let (input, _) = tag("fill_rect")(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, x) = digit1(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = digit1(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, width) = digit1(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, height) = digit1(input)?;
    let (input, _) = tag(")")(input)?;

    let x = i32::from_str(x).unwrap();
    let y = i32::from_str(y).unwrap();
    let width = i32::from_str(width).unwrap();
    let height = i32::from_str(height).unwrap();

    Ok((input, CanvasCommand::FillRect(x, y, width, height)))
}

fn parse_string(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("\"")(input)?;
    let (input, string) = alpha0(input)?;
    let (input, _) = tag("\"")(input)?;

    Ok((input, string))
}

fn parse_end(input: &str) -> IResult<&str, CanvasCommand> {
    let (input, _) = tag("end")(input)?;

    Ok((input, CanvasCommand::End))
}
