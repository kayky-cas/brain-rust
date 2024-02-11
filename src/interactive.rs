use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{self, Stylize},
    terminal::{self, WindowSize},
};

use crate::{
    parser::{Lexer, ParserMode},
    term,
};

use super::{parser::Parser, program::Program};
use std::io::{self, stdin, stdout, Write};

pub struct Interactive {
    program: Program,
    output: Vec<u8>,
    inputs: Vec<String>,
}

impl Interactive {
    pub fn new(program: Program) -> Self {
        Interactive {
            program,
            output: Vec::new(),
            inputs: Vec::new(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let _stdin = stdin();
        let mut line = String::new();

        self.render()?;

        loop {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) => {
                    line.push(c);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }) => {
                    line.remove(line.len() - 1);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => {
                    let mut parser =
                        Parser::parse(Lexer::new(line.as_bytes()), ParserMode::Command);

                    self.program.append_instructions(&mut parser);
                    self.program.run(&mut stdin().lock(), &mut self.output)?;

                    self.inputs.push(line.clone());
                    line.clear();

                    // Stdin
                    self.render()?;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => {
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn render(&mut self) -> io::Result<()> {
        let mut stdout = stdout();

        execute!(stdout, cursor::Hide, terminal::DisableLineWrap)?;

        let WindowSize { rows, columns, .. } = terminal::window_size()?;

        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

        let pos = term::Vec2::new(0, 0);
        let size = term::Vec2::new(columns, rows);

        // Borders
        term::write_box(
            &mut stdout,
            style::Color::White,
            pos,
            size,
            Some("Brain Rust"),
        )?;

        // Buffer box

        let mut buffer_text = String::new();
        let mut buffer_idx_text = String::new();
        let mut buffer_char_text = String::new();

        for (idx, &cell) in self.program.cells().iter().enumerate() {
            buffer_text.push_str(&format!("{:0>3}", cell));

            if idx == self.program.pointer() {
                buffer_idx_text.push_str(" V ");
            } else {
                buffer_idx_text.push_str(&format!("{:0>3}", idx));
            }

            let cell = cell as char;

            match cell {
                '\n' => buffer_char_text.push_str(" \\n"),
                '\t' => buffer_char_text.push_str(" \\t"),
                '\r' => buffer_char_text.push_str(" \\r"),
                '\0' => buffer_char_text.push_str(" \\0"),
                _ => buffer_char_text.push_str(&format!("  {}", cell)),
            }

            if idx < self.program.cells().len() - 1 {
                buffer_text.push_str(" | ");
                buffer_idx_text.push_str("   ");
                buffer_char_text.push_str("   ");
            }
        }

        execute!(
            stdout,
            cursor::MoveTo(5, 2),
            style::PrintStyledContent(buffer_idx_text.white())
        )?;

        term::write_box(
            &mut stdout,
            style::Color::Red,
            term::Vec2::new(3, 3),
            term::Vec2::new((buffer_text.len() + 4) as u16, 3),
            Some("Buffer"),
        )?;

        execute!(
            stdout,
            cursor::MoveTo(5, 6),
            style::PrintStyledContent(buffer_char_text.white())
        )?;

        execute!(
            stdout,
            cursor::MoveTo(5, 4),
            style::PrintStyledContent(buffer_text.with(style::Color::Red))
        )?;

        // Output box

        let output_size = 50;

        term::write_box(
            &mut stdout,
            style::Color::Green,
            term::Vec2::new(columns - output_size - 3, 3),
            term::Vec2::new(output_size, rows - 6),
            Some("Output"),
        )?;

        let mut lines = String::new();

        for (idx, x) in self.output.iter().enumerate() {
            if idx as u16 % (output_size - 4) == 0 {
                lines.push('\n');
            }
            lines.push(*x as char);
        }

        for (idx, line) in lines.lines().enumerate() {
            execute!(
                stdout,
                cursor::MoveTo(columns - output_size - 1, 4 + idx as u16),
                style::PrintStyledContent(line.white())
            )?;
        }

        // Input box
        term::write_box(
            &mut stdout,
            style::Color::White,
            term::Vec2::new(0, rows - 3),
            term::Vec2::new(columns, 3),
            Some("Input"),
        )?;

        execute!(
            stdout,
            cursor::MoveTo(2, rows - 2),
            style::PrintStyledContent(">".white()),
            cursor::MoveTo(4, rows - 2),
            cursor::Show
        )?;

        stdout.flush()?;

        Ok(())
    }
}
