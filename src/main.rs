use chrono::Local;
use color_eyre::eyre::{self, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,terminal::{ disable_raw_mode,enable_raw_mode,EnterAlternateScreen,LeaveAlternateScreen,}};
use ratatui::{layout::{Constraint, Direction, Layout},prelude::CrosstermBackend,style::Stylize,text::Line,widgets::{Block, Paragraph},Frame,Terminal,};
use ratatui_image::{ picker::Picker,protocol::StatefulProtocol,StatefulImage,};
use std::{fs,io::{self, stdout},};

pub struct App {
    running: bool,
    birthday: String,
    image: StatefulProtocol,
    image2: StatefulProtocol,
    image3: StatefulProtocol,
    image4:StatefulProtocol,
    is_luffy_default:bool,
    is_sanji: bool,
    counter: u8,
    
}

impl App {
    pub fn new(
        // these are both the images or not the images but like the things to hold the images
        initial_image_protocol: StatefulProtocol,
        img2: StatefulProtocol,
        img3: StatefulProtocol,
        img4: StatefulProtocol
        ) -> Self {
        App {
            running: true,
            birthday: together(),
            image: initial_image_protocol,
            image2: img2,
            image3:img3,
            image4:img4,
            is_luffy_default: true,
            is_sanji: true,
            counter: 0,
           

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
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25)
            ],
        )
        .split(frame.area());

        let text_content = format!(
            "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.\n\n\
            Birthday Info: {}
            Counter {}",
            self.birthday, self.counter
        );
       

        frame.render_widget(
            Paragraph::new(text_content)
                .block(Block::bordered().title(title_line.clone()))
                .centered(),
            main_layout[0],
        );


        let image_widget = StatefulImage::default();
        frame.render_stateful_widget(image_widget, main_layout[1], &mut self.image);
        
        let image_widget1 = StatefulImage::default();
        frame.render_stateful_widget(image_widget1,main_layout[0], &mut self.image2);

        let image_widget2 = StatefulImage::default();
        frame.render_stateful_widget(image_widget2, main_layout[2], &mut self.image3);

        let image_widget3 = StatefulImage::default();
        frame.render_stateful_widget(image_widget3, main_layout[3], &mut self.image4);

    }

    // so what il do it il have a folder for each day in teh assets folder
    // so like assets/january1st
    // and then inside that folder there will be 1.png and 2.png for each use use respective
    // character
    //
   
         pub fn toggle_image(&mut self) -> Result<()> {
      
        let next_path = if self.is_luffy_default {
            "./assets/luffy2.png"
        } else {
            "./assets/luffy.png"
        };
        
        // basically just decoding the next image before switching to it
        // also since i want to change the state of something in the App struct
        // i need to make it a method on the App struct or else how would i be able to acccess the
        // self stuff
        // // btw its deadass not even doing that its just switching the image and setting it 
        //
        // i think if i want more pictures instead of using a vector il just spam this next_path
        // and instead of a bool i can just make it a number and a match case and then if the index
        // is at the max i can just make it so that if index==max(wtvr) each press is a negative
        // now or i can just make another key the negative
        let picker = Picker::from_query_stdio()?;
        let dyn_img = image::ImageReader::open(next_path)?
            .decode()
            .map_err(|e| eyre::eyre!("Failed to decode image '{}': {}", next_path, e))?;

        self.image = picker.new_resize_protocol(dyn_img);
        self.is_luffy_default = !self.is_luffy_default; // Flip the state
        Ok(())
    }

    pub fn new_image(&mut self ) -> Result<()>{
        let next = if self.is_sanji {
            "./assets/zoro.png"
        } else {
            "./assets/sanji.png"
        };

        let picker = Picker::from_query_stdio()?;
        let dyn_img = image::ImageReader::open(next)?
            .decode()?;

        self.image2 = picker.new_resize_protocol(dyn_img);
        self.is_sanji = !self.is_sanji;
        Ok(())
            // question can this bool not be local? becayse this function decided whast showing not
            // the bool the bool is just the switch  i mean it would be same difference but would
            // be less configuration// ok i tried it and baiscally its fucked because you need to
            // call a function from outside the app impl and thats fucked because you dont get and
            // of the self. so its cooked just keep that tbh

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
            (_,KeyCode::Char('w')) => {
               
                self.increment_counter()
            }
            (_,KeyCode::Char('s'))=> {
                self.new_image().expect("nothing");
            }
            _ => {}
        }
    }
    fn increment_counter(&mut self) {
        self.counter+=1;
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


    // zoro sanh
    let picker1 = Picker::from_query_stdio()?;
    let dyn_img1 = image::ImageReader::open("./assets/sanji.png")?
        .decode()?;
    let image_protocol1 = picker1.new_resize_protocol(dyn_img1);


    //chopp
    let chopper_picker = Picker::from_query_stdio()?;
    let chipper = "chopper.png";
    let path = format!("./assets/{}",chipper);
    let chop_img = image::ImageReader::open(path)?
        .decode()?;
    let chop_img_pro = chopper_picker.new_resize_protocol(chop_img);
    // so chopper is basically connected to the self.image becasyse the 3rd protocaol which is
    // passed to the function the App impl up top is the protocol which is connected to the
    // self.image3 and right here is the same as we are passing chop_img_pro as the 3rd arguement
    //

    let birth_today = Picker::from_query_stdio()?;
    let path1 = today_date();
    let path = format!("./assets/{}/1.png",path1);
    let birth_img = image::ImageReader::open(path)?
        .decode()?;
    let birth_img_protocol = birth_today.new_resize_protocol(birth_img);
    



    let app = App::new(image_protocol,image_protocol1,chop_img_pro,birth_img_protocol);
    let result = app.run(terminal);

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    execute!(stdout(), crossterm::cursor::Show)?;

    result
}


