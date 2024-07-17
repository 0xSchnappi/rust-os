/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-07-17 09:40:07
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-07-17 21:43:20
 * @FilePath: /rust-os/src/main.rs
 * @Description: main
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */

#![no_std]
#![no_main]

use core::panic::PanicInfo;

// 这个函数将在panic时调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]    // 禁用名称重整，这个函数使用C语言的调用约定，而不是rust语言的调用约定
pub extern "C" fn _start() -> ! {
    loop {}
}
