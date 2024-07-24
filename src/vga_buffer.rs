/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-07-24 10:18:45
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-07-24 20:21:36
 * @FilePath: /rust-os/src/vga_buffer.rs
 * @Description:
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */

/*
 * 易失性
 * 我们在像VGA缓冲区写入是，编译器会认为我们只有往内存里写，但是没有读过，会认为是无用操作，会优化掉
 */

use volatile::Volatile;

#[allow(dead_code)] // 禁止对未使用的变量发出警告
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
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foregroud: Color, backgroud: Color) -> ColorCode {
        ColorCode((backgroud as u8) << 4 | (foregroud as u8))
    }
}

// 字符缓冲区
// 字符单元
// Bit(s)               Value
// 0-7                  ASCII code point
// 8-11                 Foregroud color     // 字符显示颜色
// 12-14                Backgroud color     // 背景颜色
// 15                   Blink       // 是否闪烁

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// 使用Volatile为了解决易失性问题
#[repr(transparent)] // 确保单个成员和类型有相同的内存布局
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT], //定义二维数组
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    // 不应该是self.column_position >= BUFFER_WIDTH-1吗
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // 可以是能打印的ASCIII字符，也可以是换行
                0x20..=0x70 | b'\n' => self.write_byte(byte),
                // 不包含在上述范围之内的字符
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn new_line(&mut self) {
        todo!()
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
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("elo ");
    writer.write_string("world");
    use core::fmt::Write;
    write!(writer, "The numbers are {} and {}", 42, 1.0 / 3.0).unwrap();
}
