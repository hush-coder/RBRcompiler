// 1. 本地包含 LALRPOP 生成的解析器
// 根据您之前 build.rs 中 lalrpop::process_root() 的运行结果，解析器文件路径为 /parser/grammar.rs
#[allow(clippy::all)]
mod grammar {
    include!(concat!(env!("OUT_DIR"), "/parser/grammar.rs"));
}

// 2. 修正 ASTNode 的导入：
// 假设 ASTNode 定义在您的主库 `compiler` 的 `parser::ast` 模块中。
// 如果您的主库在 Cargo.toml 中正确配置了依赖，此行将正确工作。
// 如果仍然报错，请检查您的主库是否将 `parser::ast::ASTNode` 设为了 `pub` 公开。
use compiler::parser::ast::ASTNode; 

// 3. 修正 CompUnitParser 的导入：
// 现在 CompUnitParser 通过本地的 `grammar` 模块访问。
use crate::grammar::CompUnitParser; 

fn main() {
    println!("🔍 验证LALR(1)语法规则运用");
    println!("{}", "=".repeat(50));
    
    // ... (以下是您的测试代码，保持不变) ...
    
    // 测试1: 验证CompUnit规则 - 程序由函数列表组成
    let test1 = r#"
        int main() {
            return 42;
        }
    "#;
    
    println!("测试1: CompUnit规则 - 程序结构");
    println!("输入: {}", test1.trim());
    match CompUnitParser::new().parse(test1) {
        Ok(ast) => {
            println!("✅ CompUnit解析成功!");
            if let ASTNode::Program(functions) = &ast {
                println!("   📋 检测到 {} 个函数", functions.len());
                for (i, func) in functions.iter().enumerate() {
                    if let ASTNode::FunctionDeclaration { name, .. } = func {
                        println!("   📝 函数 {}: {}", i + 1, name);
                    }
                }
            }
        }
        Err(e) => println!("❌ CompUnit解析失败: {}", e),
    }
    println!();
    
    // 测试2: 验证FuncDef规则 - 函数定义结构
    let test2 = r#"
        int add(int a, int b) {
            int result = a + b;
            return result;
        }
    "#;
    
    println!("测试2: FuncDef规则 - 函数定义");
    println!("输入: {}", test2.trim());
    match CompUnitParser::new().parse(test2) {
        Ok(ast) => {
            println!("✅ FuncDef解析成功!");
            if let ASTNode::Program(functions) = &ast {
                if let ASTNode::FunctionDeclaration { 
                    return_type, 
                    name, 
                    parameters, 
                    body 
                } = &functions[0] {
                    println!("   🔧 返回类型: {}", return_type);
                    println!("   📛 函数名: {}", name);
                    println!("   📥 参数数量: {}", parameters.len());
                    for (i, param) in parameters.iter().enumerate() {
                        if let ASTNode::Parameter { type_name, name } = param {
                            println!("   📋 参数 {}: {} {}", i + 1, type_name, name);
                        }
                    }
                }
            }
        }
        Err(e) => println!("❌ FuncDef解析失败: {}", e),
    }
    println!();
    
    // 测试3: 验证FunctionCall规则 - 函数调用
    let test3 = r#"
        int test() {
            int x = add(5, 3);
            return x;
        }
    "#;
    
    println!("测试3: FunctionCall规则 - 函数调用");
    println!("输入: {}", test3.trim());
    match CompUnitParser::new().parse(test3) {
        Ok(ast) => {
            println!("✅ FunctionCall解析成功!");
            // 这里可以进一步分析AST中的函数调用
        }
        Err(e) => println!("❌ FunctionCall解析失败: {}", e),
    }
    println!();
    
    println!("🎉 LALR(1)语法规则验证完成!");
    println!("💡 您的语法规则已成功转换为状态机解析器!");
}