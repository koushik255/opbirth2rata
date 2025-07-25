use color_eyre::Result;
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers}, execute, terminal::{enable_raw_mode, EnterAlternateScreen}};
use ratatui::{
    prelude::CrosstermBackend, style::Stylize, text::Line, widgets::{Block, Paragraph}, DefaultTerminal, Frame
};
use chrono::Local;
use ratatui_image::protocol::StatefulProtocol;
use std::fs;

fn main() -> color_eyre::Result<(), Box,dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout,EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let res = (|| {
        let picker = Picker::from_query_stdio()?;
        let dyn_img = image::ImageReader::open("./assets/luffy.png")?.decode()?;
        let image_protocol = picker.new_resize_protocol(dyn_img);
        let mut app = App {
            running:bool,
            birthday: String,
            image: image_protocol,
        };
    
    }

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}


/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    birthday: String,
    image: StatefulProtocol,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        App{
            running: true,
            birthday: together(),
            image:image_protocol,
        }
    }

   
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let title = Line::from("Ratatui Simple Template")
            .bold()
            .blue()
            .centered();
        let text = "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.";
        let text2 = format!(" {}",self.birthday);
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title.clone()))
                .centered(),
            frame.area(),
        );
        frame.render_widget(
            Paragraph::new(text2)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        )
       
    }

    
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}

fn together() -> String {
    read_file()
}

fn today_date() -> String{
    let today = Local::now().format("%m-%d").to_string();
    today.to_string()
     
}

fn read_file() -> String{
    let date = today_date();
    let contents = fs::read_to_string("birth.txt")
        .expect("failed to find ");
    
    let matched: String = contents
        .lines()
        .filter(|line| line.contains(&date))
        .collect();

    matched

}
