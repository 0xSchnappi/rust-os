<!--
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-07-17 09:50:34
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-07-17 21:46:47
 * @FilePath: /rust-os/docs/01-freestanding-rust-binary.md
 * @Description: 独立式可执行文件部分的说明文档
 * 
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved. 
-->
# 独立式可执行文件

## no_std属性
为了使二进制程序独立执行，不依赖操作系统，就不能使用`rust`提供的标准库
    ```rust
    // main.rs

    #![no_std]

    fn main() {
        println!("Hello, world!");
    }
    ```

## 实现panic处理函数
因为没有使用标准库，所以必须自己实`panic`
    ```rust
    // in main.rs

    use core::panic::PanicInfo;

    /// 这个函数将在panic时被调用
    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }
    ```
    
> 函数返回值为`!`，因为这个函数从不返回，所以被称为发散函数(diverging function)。
> 发散函数的返回类型称作Never类型，记为 `!`

## 栈展开
linu的libunwind和Windows的结构化异常(SEH)

## 编译命令
    ```shell
    cargo build --target thumbv7em-none-eabihf
    ```

