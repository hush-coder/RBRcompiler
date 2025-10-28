use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target, TargetTriple};
use inkwell::OptimizationLevel;
use std::path::Path;
use std::process::Command;

/// ARM 后端 - 封装 ARM 架构的 LLVM 后端逻辑
pub struct ArmBackend;

impl ArmBackend {
    /// 执行从 LLVM IR 到最终可执行文件的编译和链接
    pub fn compile_and_link(module: &Module, output_path: &Path) -> Result<(), String> {
        // --- 1. 初始化和配置目标机器 ---
        
        // 初始化所有目标
        Target::initialize_all(&InitializationConfig::default());

        // 创建目标三元组 - 根据当前系统架构选择
        let target_triple = if cfg!(target_arch = "aarch64") {
            "aarch64-unknown-linux-gnu"
        } else if cfg!(target_arch = "x86_64") {
            "x86_64-unknown-linux-gnu"
        } else if cfg!(target_arch = "arm") {
            "arm-unknown-linux-gnueabihf"
        } else {
            "x86_64-unknown-linux-gnu" // 默认使用x86_64
        };
        
        let triple = TargetTriple::create(target_triple);
        
        // 获取 ARM 目标
        let target = Target::from_triple(&triple)
            .map_err(|e| format!("Failed to create ARM target: {}", e))?;
        
        // 创建目标机器
        let target_machine = target
            .create_target_machine(
                &triple,
                "generic", // CPU name
                "",        // Features
                OptimizationLevel::Default, // 优化级别
                inkwell::targets::RelocMode::Default,
                inkwell::targets::CodeModel::Default,
            )
            .ok_or_else(|| "Failed to create target machine".to_string())?;

        // --- 2. 验证模块 ---
        if let Err(validation_error) = module.verify() {
            return Err(format!("LLVM module validation failed: {}", validation_error));
        }

        // --- 3. 生成目标文件 (.o) ---
        let obj_file_path = output_path.with_extension("o");
        target_machine
            .write_to_file(
                module,
                inkwell::targets::FileType::Object,
                &obj_file_path,
            )
            .map_err(|e| format!("Failed to write object file: {:?}", e))?;

        println!("ARM Object file created at: {:?}", obj_file_path);

        // --- 4. 链接 ---
        
        // 使用系统链接器
        let linker = "gcc";
        
        let status = Command::new(linker)
            .arg(&obj_file_path)
            .arg("-o")
            .arg(output_path)
            .arg("-static") // 静态链接
            .status()
            .map_err(|e| format!("Failed to execute linker {}: {}", linker, e))?;

        if status.success() {
            println!("Successfully linked executable: {:?}", output_path);
            Ok(())
        } else {
            Err(format!("Linking failed. Linker status: {:?}", status))
        }
    }
    
    /// 生成 ARM 汇编代码（不链接）
    pub fn generate_assembly(module: &Module) -> Result<String, String> {
        // 初始化所有目标
        Target::initialize_all(&InitializationConfig::default());

        // 创建目标三元组 - 根据当前系统架构选择
        let target_triple = if cfg!(target_arch = "aarch64") {
            "aarch64-unknown-linux-gnu"
        } else if cfg!(target_arch = "x86_64") {
            "x86_64-unknown-linux-gnu"
        } else if cfg!(target_arch = "arm") {
            "arm-unknown-linux-gnueabihf"
        } else {
            "x86_64-unknown-linux-gnu" // 默认使用x86_64
        };
        
        let triple = TargetTriple::create(target_triple);
        
        // 获取 ARM 目标
        let target = Target::from_triple(&triple)
            .map_err(|e| format!("Failed to create ARM target: {}", e))?;
        
        // 创建目标机器
        let target_machine = target
            .create_target_machine(
                &triple,
                "generic",
                "",
                OptimizationLevel::Default,
                inkwell::targets::RelocMode::Default,
                inkwell::targets::CodeModel::Default,
            )
            .ok_or_else(|| "Failed to create target machine".to_string())?;

        // 验证模块
        if let Err(validation_error) = module.verify() {
            return Err(format!("LLVM module validation failed: {}", validation_error));
        }

        // 生成汇编代码到临时文件
        let temp_asm_path = std::env::temp_dir().join("temp_assembly.s");
        target_machine
            .write_to_file(
                module,
                inkwell::targets::FileType::Assembly,
                &temp_asm_path,
            )
            .map_err(|e| format!("Failed to generate assembly: {:?}", e))?;

        // 读取生成的汇编文件内容
        let assembly = std::fs::read_to_string(&temp_asm_path)
            .map_err(|e| format!("Failed to read assembly file: {}", e))?;

        // 清理临时文件
        let _ = std::fs::remove_file(&temp_asm_path);

        Ok(assembly)
    }
}
