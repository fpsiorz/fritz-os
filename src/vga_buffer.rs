#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Copy, Clone)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Copy, Clone)]
struct ScreenChar {
    character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    col_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        if self.col_pos >= BUFFER_WIDTH {
            self.new_line();
        }
        let row = BUFFER_HEIGHT - 1;
        let col = self.col_pos;
        self.buffer.chars[row][col].write(ScreenChar {
            character: byte,
            color_code: self.color_code,
        });
        self.col_pos += 1;        
    }

    pub fn write_char(&mut self, character: char) {
        match character {
            '\n' => self.new_line(),
            '\t' => while self.col_pos % 8 != 0 {
                self.write_byte(b' ');
            },
            ' '...'~' => self.write_byte(character as u8),
            _ => self.write_byte(6), // spades symbol because why not
            // TODO actually translate symbols that are in codepage 437
        }
    }

    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let c = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(c);
            }
        }
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[BUFFER_HEIGHT - 1][col].write(ScreenChar{
                character: b' ',
                color_code: self.color_code,
            });
        }
        self.col_pos = 0;
    }
}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        col_pos: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)}, 
    });
}

macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::print(format_args!($($arg)*)));
}

macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

