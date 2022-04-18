use std::net::TcpListener;
use std::thread;
use std::io::{BufRead, BufWriter, Read, Write};
use std::io::BufReader;
use std::sync::mpsc;
use std::process::Command;
use std::str::FromStr;

use macroquad::prelude::*;

mod config;
mod parser;

#[derive(Eq, PartialEq, Clone)]
pub enum CanvasCommand {
    AskInput,
    FillStyle(String),
    FillRect(i32, i32, i32, i32),
    End
}

#[macroquad::main("pra2do-terminal")]
async fn main() {
    let config = config::load_config();

    let (tx, rx) = mpsc::channel();
    let listener = TcpListener::bind(config.address).unwrap();

    let cli = shlex::split(&config.exec).unwrap();
    let (exe, args) = cli.split_first().unwrap();
    let prolog = Command::new(exe)
        .args(args)
        .spawn()
        .expect("Can't run Prolog");

    thread::spawn(move || {
        let mut buffer = Vec::new();
        let (socket, _addr) = listener.accept().unwrap();
        let mut reader = BufReader::new(&socket);
        let mut writer = BufWriter::new(&socket);
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            if let Ok((_, command)) = parser::parse_clause(&line) {
                buffer.push(command.clone());
                match command {
                    CanvasCommand::End => {
                        tx.send(buffer.clone()).unwrap();
                        buffer.clear();
                    }
                    CanvasCommand::AskInput => {
                        let right = is_key_down(KeyCode::Right);
                        let left = is_key_down(KeyCode::Left);
                        let up = is_key_down(KeyCode::Up);
                        let down = is_key_down(KeyCode::Down);

                        let mut input = String::new();
                        input.push_str("[");
                        let mut input_vec = Vec::new();
                        if right {
                            input_vec.push("right");
                        }
                        if left {
                            input_vec.push("left");
                        }
                        if up {
                            input_vec.push("up");
                        }
                        if down {
                            input_vec.push("down");
                        }
                        input.push_str(&input_vec.join(","));
                        input.push_str("].\n");

                        writer.write(input.as_bytes());
                        println!("{}", input);
                        writer.flush();
                    }
                    _ => {}
                }
            }
        }
    });
    let mut commands = Vec::new();
    loop {
        while let Ok(buffer) = rx.try_recv() {
            commands = buffer;
        }

        clear_background(RED);

        let mut color = RED;

        for command in commands.iter() {
            match command {
                CanvasCommand::AskInput => {},
                CanvasCommand::FillStyle(req_color) => {
                    color = match req_color.as_str() {
                        "green" => GREEN,
                        "blue" => BLUE,
                        "red" => RED,
                        _ => BLUE,
                    };
                },
                CanvasCommand::FillRect(x, y, width, height) => {
                    draw_rectangle(*x as f32, *y as f32, *width as f32, *height as f32, color);
                }
                CanvasCommand::End => next_frame().await
            }
        }
    }
}