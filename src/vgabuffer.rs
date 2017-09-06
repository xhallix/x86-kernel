use core::ptr::Unique;
use core::fmt;
use volatile::Volatile;


#[allow(dead_code)]
#[repr(u8)]
pub enum Color  {
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

#[derive(Debug, Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> ColorCode{
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_char : u8,
    pub color_code : ColorCode,
}

const BUFFER_HEIGHT : usize = 25;
const BUFFER_WIDTH : usize = 80;

pub struct Buffer {
    pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


pub struct Writer {
    pub column_pos : usize,
    pub color_code : ColorCode,
    pub buffer : Unique<Buffer>, // Unique creates a hashmap
}

impl Writer {
    pub fn write_byte(&mut self, byte : u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_pos;

                let color_code = self.color_code;
                self.buffer().chars[row][col].write(ScreenChar {
                    ascii_char : byte,
                    color_code : color_code,
                });
                self.column_pos += 1;
            }
        }
    }

    pub fn write_str(&mut self, word : &str) {
        for byte in word.bytes() {
            self.write_byte(byte)
        }
    }

    pub fn buffer(&mut self) -> &mut Buffer {
        unsafe{ self.buffer.as_mut() }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_WIDTH {
            for col in 0..BUFFER_HEIGHT {
                let buffer = self.buffer();
                let character = buffer.chars[row][col].read();
                buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT-1);
        self.column_pos = 0;
    }

    pub fn clear_screen(&mut self) {
       // IMPLEMENT
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer().chars[row][col].write(blank);
        }
    }
}