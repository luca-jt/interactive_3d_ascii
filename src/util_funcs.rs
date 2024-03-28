use std::io::{Write, Read, stdin, stdout};
use crossterm::event::{self, KeyCode, KeyEvent, Event};
use std::time::Duration;


pub fn print_screen(frame_buffer: &Vec<Vec<char>>)
{
    print!("\x1B[2J\x1B[H"); // clear screen
    stdout().flush().unwrap();

    for v in frame_buffer
    {
        let row_string: String = v.iter().collect();
        println!("{}", row_string);
    }
}


pub fn space_pressed() -> bool
{
    loop
    {
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                if code == KeyCode::Char(' ').into() {
                    return true;
                }
            }
        }
        break;
    }
    false
}


pub fn e_pressed() -> bool
{
    loop
    {
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                if code == KeyCode::Char('e').into() {
                    return true;
                }
            }
        }
        break;
    }
    false
}


pub fn pause()
{
    let mut out = stdout();
    out.write(b"Press Enter to quit...").unwrap();
    out.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}


pub fn clear_frame_buffer(buffer: &mut Vec<Vec<char>>)
{
    for row in buffer.iter_mut()
    {
        for ch in row.iter_mut()
        {
            *ch = ' ';
        }
    }
}


pub fn clear_z_buffer(buffer: &mut Vec<f32>)
{
    for entry in buffer.iter_mut()
    {
        *entry = 0.0;
    }
}
