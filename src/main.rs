/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-07-17 09:40:07
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-07-24 20:14:51
 * @FilePath: /rust-os/src/main.rs
 * @Description: main
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// 这个函数将在panic时调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // 禁用名称重整，这个函数使用C语言的调用约定，而不是rust语言的调用约定
pub extern "C" fn _start() -> ! {
    // let vag_buffer = 0xb8000 as *mut u8;
    // // VGA 文本缓冲区
    // // 地址0xb8000
    // // 通常有25行组成，每行包含80列共2000个字符单元

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         // 裸指针必须在unsafe里执行
    //         *vag_buffer.offset(i as isize * 2) = byte;
    //         *vag_buffer.offset(i as isize * 2 + 1) = 0xb; // 0xb代表淡青色
    //     }
    // }
    vga_buffer::print_something();
    loop {}
}
