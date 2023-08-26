#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;



// TODO: Implement macro println_prefix.
#[cfg(feature = "axstd")]
use axstd::println_prefix;

#[cfg_attr(feature = "axstd", no_mangle)]
#[macro_export]
macro_rules! println_prefix {
    ($prefix:expr, $($arg:tt)*) => {
        // 在输出前添加前缀并使用println!宏进行格式化输出
        println!(concat!($prefix, "{}"), $($arg)*);
    };
}

fn main() {
    println!("Hello, world!");

    let times = 2;
    println_prefix!("Stdout: ", "Hello, world![{}]", times);

    println!("\n[ArceOS Tutorial]: A0 okay!");
}
