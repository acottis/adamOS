#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    Gray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
// This structure holds the bits for fore/back colour
struct ColorCode(u8);

// Caluculate bits for fore/back from the decimal in the enum Color then
// stores into ColorCode(u8)
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        // 0000 0000 Background is set first, then the bits are shifted 4 to the right then
        // the we or that with the bits from foreground to fill the now empty first 4 bits
        // 0000 0000
        // 0000 1010
        // 1010 0000 | 1010 0101
        // 1010 0101
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    fn write_byte(&mut self, byte: u8) {
        if self.column_position >= BUFFER_WIDTH {
            self.new_line()
        }
        let row = self.row_position;
        let col = self.column_position;

        self.buffer.chars[row][col] = ScreenChar {
            ascii_character: byte,
            color_code: self.color_code,
        };
        self.column_position += 1;
    }
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            let current_row = self.buffer.chars[row];
            self.clear_line(row);
            self.buffer.chars[row - 1] = current_row;
            self.column_position = BUFFER_WIDTH - BUFFER_WIDTH;
        }
    }

    // Clears the line by printing back text to the row
    fn clear_line(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = ScreenChar {
                ascii_character: 0x00,
                color_code: self.color_code,
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // 0x20 - 0x7e are valid ascii characters to print
                b'\n' => self.new_line(),
                0x20..=0x7e => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: BUFFER_WIDTH - BUFFER_WIDTH,
        row_position: BUFFER_HEIGHT - 1,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_string("H\n");
    writer.write_string("ello \n안녕하세요");
    writer.write_string("Wörld! NAOMI\n");
    write!(writer, "The numbers are {} \nand {}", 42, 1.0 / 3.0);
}

// pub static WRITER: Writer = Writer {
//     column_position: BUFFER_WIDTH - BUFFER_WIDTH,
//     row_position: BUFFER_HEIGHT - 1,
//     color_code: ColorCode::new(Color::Green, Color::Black),
//     buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
// };