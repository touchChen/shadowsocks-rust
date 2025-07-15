use std::process::ExitCode;
use tokio::runtime::{self, Runtime};

pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tc {
    use super::*;
    #[test]
    fn test_add() {
        println!("start learning Rust");
        assert_eq!(add(2, 3), 5); // 验证加法
        assert_ne!(add(0, 0), 1);  // 验证不等
    }
}


// 自定义错误类型
#[derive(Debug)]
struct AppError {
    message: String,
}

impl AppError {
    fn new(msg: &str) -> Self {
        AppError {
            message: msg.to_string(), // 使用字段
        }
    }

    fn exit_code(&self) -> u8 {
        1
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Application Error: {}", self.message)
    }
}


#[derive(Debug)]
pub enum RuntimeError {
    
}


async fn main_async_operation(succeed: bool) -> Result<(), AppError> {
    if succeed {
        println!("Operation succeeded!");
        Ok(())
    } else {
        Err(AppError::new("Failed to perform operation"))
    }
}

fn get_runtime() -> Result<Runtime,RuntimeError>{
    // 创建Tokio运行时
    let runtime = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    Ok(runtime)
}


fn run_app(succeed: bool) -> ExitCode {
    match get_runtime(){
        Ok(runtime)=>{
            match runtime.block_on(main_async_operation(succeed)) {
                Ok(()) => {
                    println!("Exiting successfully");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    println!("{:#?}", err);  // 打印错误信息
                    ExitCode::from(err.exit_code())
                }
            }
        }
        Err(_)=>{
            ExitCode::FAILURE
        }
    }
}


fn main_features() {
    println!("启用的特性:");
    
    // 检测特定特性
    if cfg!(feature = "local-online-config") {
        println!("- local-online-config");
    }
    
    if cfg!(feature = "logging") {
        println!("- logging");
    }
      
    // 检测默认特性
    if cfg!(feature = "default") {
        println!("- default");
    }
}



fn main() {
    main_features();
    let mut c = 99;

    #[cfg(feature = "local-online-config")]
    {
        c = c +1;
    }
    
    println!("{}",c);
    // 第一次运行（成功）
    println!("--- Running with success ---");
    let exit_code = run_app(true);
    let exit_value = if ExitCode::SUCCESS == exit_code { 0 } else { 1 };
    println!("{}",exit_value);

    println!("\n--- Running with failure ---");
    let exit_code = run_app(false);
    let exit_value = if ExitCode::SUCCESS == exit_code { 0 } else { 1 };
    println!("{}",exit_value);
}
