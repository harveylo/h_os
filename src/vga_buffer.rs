use volatile::Volatile;
use core::fmt;
use core::fmt::Write;

// Rust compiler will warn each unused variable, add this attribute to avoid such warnings
#[allow(dead_code)]
// Derived form those traits
// allow this type to be coppyed, printed and comparaed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// align as uint8, each variant can only range from 0 to 255
#[repr(u8)]
pub enum Color{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
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
// this repr can only be applied to those struct has only one field(and multiple size 0, alignment 1(like PhatomData<T> fields))
// to make the whole struct/enum have exactly the same ABI and layout as the one field
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode{
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Rust compiler does not guarantee the layout order of each fileds
// also does not guarantee the same order with each compilation
// add this attribute to use C-like layout, which guarantees the order and the same order on each compilation
#[repr(C)]
struct ScreenChar{
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer{
    // Use Volatile to wrap ScreenChar
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    // lifetime annotation
    // 'static annotation denotes this reference should be valid during the whole program execution
    buffer: &'static mut Buffer,
}

impl Writer{
    pub fn write_byte(&mut self, byte: u8){
        match byte {
            b'\n' => self.new_line(),
            x => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT -1;
                let col = self.column_position;

                let color_code = self.color_code;
                // Volatile variable can not be accessed directly
                // Use write method to do so
                self.buffer.chars[row][col].write(ScreenChar{
                    color_code,
                    ascii_character: x,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        // BUFFER_HEIGHT not included
        for row in 1..BUFFER_HEIGHT{
            for col in 1..BUFFER_WIDTH{
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col] .write(character);
            }
        }

        self.clear_row(BUFFER_HEIGHT-1);
        self.column_position = 0;
    }
    
    fn clear_row(&mut self, row: usize){
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(ScreenChar { ascii_character: (0xff), color_code: (self.color_code) });
        }
    }

    pub fn write_string(&mut self, s: &str){
        for byte in s.bytes() {
            match byte {
                // In the range from 0x20 to 0x7e(inclusive), those characters are printable
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // if not a printable character, print 0xfe
                _ => self.write_byte(0xfe),
            }
        }
    }
    pub fn write_hollow_smiling(&mut self) {
        self.write_byte(0x01);
    }
    pub fn write_opaque_smiling(&mut self){
        self.write_byte(0x02);
    }
    pub fn write_heart(&mut self){
        self.write_byte(0x03);
    }

}

// Implement the fmt::Write trait for Writer structure
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub fn print_something(){
    let mut writer = Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Blue, Color::Black),
        // First, convert 0xb8000 as a raw pointer of Buffer pointing to address 0xb8000,
        // then dereference this raw pointer, pass it as a muttable reference to buffer
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
    };

    writer.write_byte(b'H');
    writer.write_string("ello, ");
    writer.write_string("rust os World!");
    writer.write_heart();
    writer.write_hollow_smiling();
    writer.write_opaque_smiling();
    // The return value of write! must be used
    // Therefore, call the unwrap function, this function will panic when error ocurrs
    write!(writer, "The numbers are {} and {}",42, 1.0/3.0).unwrap();
}