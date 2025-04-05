use crossterm::terminal::{enable_raw_mode,disable_raw_mode};
use crossterm::event::{read, Event::Key, KeyCode::Char};
pub struct  Editor{

}

impl Editor{
    pub fn default()-> Self{
        Editor{}
    }
    pub fn run(&self) { 
        if let Err(err) = self.repl() { 
            panic!("{err:#?}");
    }
    print!("Goodbye.\r\n");
}
    pub fn repl(&self)->Result<(),std::io::Error>{
        match enable_raw_mode() {
            Ok(()) => println!("Raw mode Enalbed"),
            Err(e) => {
                eprintln!("Failed to enable raw mode: {}", e);
            }
        }
        loop { 
            match read() {
                Ok(Key(event)) => { 
                    println!("{event:?} \r"); 
                    if let Char(c) = event.code { 
                        if c == 'q' {
                            break;
                        }
                    } 
                },
                Err(err) => println!("Error: {}", err),
                _ => ()}
            }
            disable_raw_mode()?;
            Ok(())
    }
}
  

