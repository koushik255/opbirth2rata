use chrono::Local;
use color_eyre::eyre::{self, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::CrosstermBackend,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
    Terminal,
};
use ratatui_image::{
    picker::Picker,
    protocol::StatefulProtocol,
    StatefulImage,
};
use std::{
    fs,
    io::{self, stdout},
};

pub struct App {
    running: bool,
    birthday: String,
    image: StatefulProtocol,
    is_luffy_default:bool,
}

impl App {
    pub fn new(initial_image_protocol: StatefulProtocol) -> Self {
        App {
            running: true,
            birthday: together(),
            image: initial_image_protocol,
            is_luffy_default: true,

        }
    }

    pub fn run(mut self,mut terminal: Terminal<CrosstermBackend<impl io::Write>>,) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
        
            
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let title_line = Line::from("Ratatui Simple Template").bold().blue().centered();

        let main_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ],
        )
        .split(frame.area());

        let text_content = format!(
            "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.\n\n\
            Birthday Info: {}",
            self.birthday
        );

        frame.render_widget(
            Paragraph::new(text_content)
                .block(Block::bordered().title(title_line.clone()))
                .centered(),
            main_layout[0],
        );

        let image_widget = StatefulImage::default();
        frame.render_stateful_widget(image_widget, main_layout[0], &mut self.image);
    }

    // so what il do it il have a folder for each day in teh assets folder
    // so like assets/january1st
    // and then inside that folder there will be 1.png and 2.png for each use use respective
    // character
    //
   
         pub fn toggle_image(&mut self) -> Result<()> {
        let _current_path = if self.is_luffy_default {
            "./assets/luffy.png"
        } else {
            "./assets/luffy2.png"
        };
        
        let next_path = if self.is_luffy_default {
            "./assets/luffy2.png"
        } else {
            "./assets/luffy.png"
        };
        
        // basically just decoding the next image before switching to it
        // also since i want to change the state of something in the App struct
        // i need to make it a method on the App struct or else how would i be able to acccess the
        // self stuff
        let picker = Picker::from_query_stdio()?;
        let dyn_img = image::ImageReader::open(next_path)?
            .decode()
            .map_err(|e| eyre::eyre!("Failed to decode image '{}': {}", next_path, e))?;

        self.image = picker.new_resize_protocol(dyn_img);
        self.is_luffy_default = !self.is_luffy_default; // Flip the state
        Ok(())
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(250))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }

   
     
    Ok(())
            }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_,KeyCode::Char('k'))=>  {
                if let Err(e) = self.toggle_image() {
                    eprintln!("Error toggling image: {}",e);
                }
            },
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}

fn together() -> String {
    read_file().unwrap_or_else(|_| "No birthday found or file error.".to_string())
}

fn today_date() -> String {
    Local::now().format("%m-%d").to_string()
}

fn read_file() -> Result<String> {
    let date = today_date();
    let contents = fs::read_to_string("birth.txt")
        .map_err(|e| eyre::eyre!("Failed to read birth.txt: {}", e))?;

    let matched_lines: Vec<&str> = contents
        .lines()
        .filter(|line| line.contains(&date))
        .collect();

    Ok(if matched_lines.is_empty() {
        "No birthdays today!".to_string()
    } else {
        matched_lines.join("\n")
    })
}

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;
    let mut stdout_handle = stdout();
    execute!(stdout_handle, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout_handle);
    let terminal = Terminal::new(backend)?;

    let picker = Picker::from_query_stdio()?;
    let dyn_img = image::ImageReader::open("./assets/luffy.png")?
        .decode()
        .map_err(|e| eyre::eyre!("Failed to decode image: {}", e))?;
    let image_protocol = picker.new_resize_protocol(dyn_img);

    let app = App::new(image_protocol);
    let result = app.run(terminal);

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    execute!(stdout(), crossterm::cursor::Show)?;

    result
}


