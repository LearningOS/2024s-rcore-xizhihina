#![no_std]
#![no_main]

mod lang_items;
mod sbi;

use core::fmt;
use core::fmt::Write;
// use crate::sbi::shutdown;//这俩一样的？
use sbi::shutdown;


core::arch::global_asm!(include_str!("entry.asm"));
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    shutdown();
}


/*
#[no_mangle]：
这个属性确保 Rust 编译器不会修改函数的名称。
在 Rust 中，函数名在编译时会被 "mangle"（即改变）以包含额外的信息，如类型信息。但在与 C 语言或其他低级语言交互时，你需要保持函数名不变。

extern "C"：
这个属性告诉 Rust 编译器使用 C 语言的调用约定（calling convention）来编译这个函数。调用约定决定了函数如何被调用，包括参数如何传递、函数如何返回等。
 */
// #[no_mangle]
// extern "C" fn _start() {
//     println!("Hello, world!");
//     // sys_exit(9);
//     shutdown();
// }


///
/// 系统调用
/// 
const SYSCALL_EXIT: usize = 93;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        // asm! 宏允许我们在 Rust 代码中嵌入汇编代码。
        core::arch::asm!(  
            "ecall",              // 这是要执行的汇编指令  
            inlateout("x10") args[0] => ret,  // 输入/输出约束：x10/a0 寄存器从 args[0] 读取值，并在指令执行后将结果写入 ret  
            in("x11") args[1],    // 输入约束：x11/a1 寄存器从 args[1] 读取值  
            in("x12") args[2],    // 输入约束：x12/a2 寄存器从 args[2] 读取值  
            in("x17") id,         // 输入约束：x17 寄存器从 id 读取值  
        );
    }
    ret
}
/*
000000000001116c <_ZN2os7syscall17h2a6fcb2e627ab196E>:
; fn syscall(id: usize, args: [usize; 3]) -> isize {
   1116c: 41 11         addi    sp, sp, -0x10
   1116e: 2e 86         mv      a2, a1
   11170: aa 88         mv      a7, a0
   11172: 46 e4         sd      a7, 0x8(sp)
;             inlateout("x10") args[0] => ret,  // 输入/输出约束：x10 寄存器从 args[0] 读取值，并在指令执行后将结果写入 ret
   11174: 08 62         ld      a0, 0x0(a2)
;             in("x11") args[1],    // 输入约束：x11 寄存器从 args[1] 读取值
   11176: 0c 66         ld      a1, 0x8(a2)
;             in("x12") args[2],    // 输入约束：x12 寄存器从 args[2] 读取值
   11178: 10 6a         ld      a2, 0x10(a2)
;         core::arch::asm!(
   1117a: 73 00 00 00   ecall
   1117e: 2a e0         sd      a0, 0x0(sp)
;     ret
   11180: 02 65         ld      a0, 0x0(sp)
; }
   11182: 41 01         addi    sp, sp, 0x10
   11184: 82 80         ret

函数开头：_ZN2os7syscall17h2a6fcb2e627ab196E
    000000000001116c <_ZN2os7syscall17h2a6fcb2e627ab196E>:
        这是函数 _ZN2os7syscall17h2a6fcb2e627ab196E 的地址标签。这个标签是编译器生成的，通常包含了一些关于函数的信息（如命名空间、函数名、哈希值等）。
函数体
    1116c: 41 11 addi sp, sp, -0x10
        这条指令将栈指针（sp）减少 16 字节（0x10），为局部变量和可能的临时数据腾出空间。
    1116e: 2e 86 mv a2, a1
        将参数寄存器 a1（在 RISC-V 中通常用于传递函数的第二个参数）的值复制到 a2 寄存器。这里 a2 稍后用于索引参数数组。
    11170: aa 88 mv a7, a0
        将参数寄存器 a0（在 RISC-V 中通常用于传递函数的第一个参数）的值复制到 a7 寄存器。这里 a7 用来保存系统调用编号 id。
    11172: 46 e4 sd a7, 0x8(sp)
        将 a7 寄存器（包含系统调用编号 id）的值存储到栈上，偏移量为 8 字节（0x8(sp)）。这不是必需的，但在某些情况下，为了保持栈帧的一致性或便于调试，编译器可能会选择这样做。
    11174: 08 62 ld a0, 0x0(a2)
        从 a2 寄存器指向的地址（即参数数组的第一个元素）加载 8 字节数据到 a0 寄存器。这对应于 Rust 代码中的 args[0]。
    11176: 0c 66 ld a1, 0x8(a2)
        从 a2 寄存器指向的地址加上 8 字节（即参数数组的第二个元素）加载 8 字节数据到 a1 寄存器。这对应于 Rust 代码中的 args[1]。
    11178: 10 6a ld a2, 0x10(a2)
        从 a2 寄存器指向的地址加上 16 字节（即参数数组的第三个元素）加载 8 字节数据到 a2 寄存器。这对应于 Rust 代码中的 args[2]。
    1117a: 73 00 00 00 ecall
        执行 ecall 指令，触发系统调用。此时，a0、a1、a2 寄存器分别包含 args[0]、args[1]、args[2] 的值，而系统调用编号 id 应该在某个地方（可能是之前存储在栈上的 a7，但在这个例子中它没有被直接使用）。
        ecall 指令将权限提升到内核模式并将程序跳转到指定的地址。在这个例子中，这个地址是内核的系统调用处理程序。
        应用程序访问操作系统提供的系统调用的指令是 ecall ，操作系统访问 RustSBI提供的SBI调用的指令也是 ecall ， 虽然指令一样，但它们所在的特权级是不一样的。 简单地说，应用程序位于最弱的用户特权级（User Mode）， 操作系统位于内核特权级（Supervisor Mode）， RustSBI位于机器特权级（Machine Mode）。
    1117e: 2a e0 sd a0, 0x0(sp)
        将 a0 寄存器的值（即系统调用的返回值）存储到栈上的某个位置。这通常是为了在后续指令中能够访问这个返回值。
    11180: 02 65 ld a0, 0x0(sp)
        从栈上的位置加载 8 字节数据到 a0 寄存器。这实际上是将之前存储的系统调用返回值重新加载回 a0 寄存器，因为 RISC-V 架构通常约定使用 a0 寄存器来返回函数的结果。
函数结尾
    11182: 41 01 addi sp, sp, 0x10
        将栈指针（sp）增加 16 字节，释放之前为局部变量和临时数据分配的空间。
    11184: 82 80 ret
        返回
*/


pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

const SYSCALL_WRITE: usize = 64;

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
  syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

struct Stdout;

impl Write for Stdout {
    //猜测：write_fmt使用的是write_str
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(1, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
    //unwrap 是 Result 类型的一个方法，用于从 Result 类型中提取值并忽略任何可能的错误。如果 Result 是一个错误，unwrap 会导致程序 panic（即崩溃）。在生产代码中，通常建议使用更健壮的错误处理方式，如 match 表达式或 ? 操作符来处理 Result 类型的值。
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
    //$fmt: literal：这个片段捕获一个字符串字面量，并将其绑定到 $fmt 变量上。literal 片段类型确保你捕获的是一个硬编码的字符串字面量，而不是一个表达式或变量。
    //$(, $($arg: tt)+)?：这是一个复杂的重复片段。它捕获零个或多个以逗号分隔的参数列表。$arg: tt 意味着 $arg 可以是任何 token tree（token 树），这是宏中的基本构建块。
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
        //$crate：在宏内部，$crate 是一个特殊的宏变量，它指代定义宏的 crate。这是为了避免在宏展开时可能发生的名称冲突。
        //format_args!：这是 Rust 标准库中的一个宏，它接受一个格式字符串和一系列参数，并生成一个实现了 std::fmt::Arguments trait 的值。这个值可以被传递给任何需要格式化参数的函数或方法。
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
        //concat!：这也是 Rust 标准库中的一个宏，它接受多个字符串字面量作为参数，并将它们连接成一个新的字符串字面量。在 println! 宏中，它被用来在原始格式字符串后添加一个换行符。
    }
}

///
/// 清空 BSS 段
/// 
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}