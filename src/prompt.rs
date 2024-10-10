use std::io::{stdout, Result};

use crossterm::{cursor::{RestorePosition, SavePosition}, event::{read, Event, KeyCode, KeyEventKind}, execute, style::{Color, Print, ResetColor, SetForegroundColor, Stylize}, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}};


pub struct Prompt {
    desc: String,
    default: String,
    numbers_only: bool,
}


impl Prompt {
    

    pub fn new(desc: &str, default: &str) -> Self {
        Self {
            desc: desc.into(),
            default: default.into(),
            numbers_only: false,
        }
    }


    pub fn numbers_only(mut self) -> Self {
        self.numbers_only = true;
        self
    }


    pub fn prompt(&self) -> String {
        self._prompt()
            .expect("failed to prompt")
    }


    fn _prompt(&self) -> Result<String> {

        enable_raw_mode()?;

        let w = &mut stdout();

        println!("  {} {}", "→".blue(), self.desc);
        execute!(w, Print("    "),
                    SavePosition,
                    SetForegroundColor(Color::DarkGrey),
                    Print(&self.default),
                    ResetColor
        )?;
       
    
        let mut line = String::new();
    
        loop {
            let event = read()?;
            match event {
                Event::Key(event) if event.kind == KeyEventKind::Press => {
                    match event.code {
                        KeyCode::Char(char) => {
                            if self.numbers_only && !char.is_numeric() {
                                continue;
                            }
                            line.push(char);

                            execute!(w, RestorePosition, Clear(ClearType::UntilNewLine))?;
                            execute!(w, Print(&line))?;
                        }
                        KeyCode::Backspace => {
                            line.pop();
                            execute!(w, RestorePosition, Clear(ClearType::UntilNewLine))?;
                            
                            if !line.is_empty() {
                                println!("{}", line); 
                            } else {
                                println!("{}", self.default.clone().dark_grey());
                            }
                        }
                        KeyCode::Enter => {
                            break;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    
        if line.is_empty() {
            line.push_str(&self.default);
        }

        execute!(w,
            RestorePosition,
            Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::DarkGrey),
            Print(&line),
            ResetColor,
            Print("\n"),
        )?;
    
        disable_raw_mode()?;
    
        Ok(line)
    }

}