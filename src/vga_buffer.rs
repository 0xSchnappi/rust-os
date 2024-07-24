/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-07-24 10:18:45
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-07-24 21:26:15
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

// 全局接口
// 使用全局静态变量
use lazy_static::lazy_static; // 延迟初始化  目的是为了解决编译期初始化常量求值器和常函数问题
use spin::Mutex; // RefCell UnsafeCell 是非i同步类型，不满足Sync，所以这里使用自旋锁解决
lazy_static! {
    pub static ref WRITE: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
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
        // 将VGA矩阵所有行向上移动
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1); // 最后一行清除
        self.column_position = 0; // 重置列位置最后一个到第一个
    }

    pub fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code,
            });
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

// pub fn print_something() {

//     writer.write_byte(b'H');
//     writer.write_string("elo ");
//     writer.write_string("world");
//     use core::fmt::Write;
//     write!(writer, "The numbers are {} and {}", 42, 1.0 / 3.0).unwrap();
// }
