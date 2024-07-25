/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-07-17 09:40:07
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-07-25 21:26:36
 * @FilePath: /rust-os/src/main.rs
 * @Description: main
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
#![no_std]
#![no_main]
// 自定义单元测试
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"] // 没有解决使用panic = "abort" 情况cargo test报错duplicate langitem问题

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Runing {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    println!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

use core::panic::PanicInfo;

mod vga_buffer;

// 这个函数将在panic时调用
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
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

    // use core::fmt::Write;
    // vga_buffer::WRITE.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITE.lock(), ", some number:{} {}", 42, 1.337).unwrap();
    println!("Hello World{}", "!");
    // panic!("print panic");

    #[cfg(test)]
    test_main();

    loop {}
}
