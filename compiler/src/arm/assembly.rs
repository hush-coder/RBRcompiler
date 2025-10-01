use std::fs;
use std::io::Write;

pub fn write_assembly<S: AsRef<str>>(path: &str, asm: S) -> std::io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(asm.as_ref().as_bytes())?;
    Ok(())
}

pub fn format_assembly(asm: &str) -> String {
    let mut formatted = String::new();
    let lines: Vec<&str> = asm.lines().collect();
    
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            formatted.push('\n');
            continue;
        }
        
        // 添加适当的缩进
        if trimmed.ends_with(':') {
            // 标签不需要缩进
            formatted.push_str(trimmed);
        } else if trimmed.starts_with('#') {
            // 注释
            formatted.push_str(&format!("    {}", trimmed));
        } else {
            // 指令需要缩进
            formatted.push_str(&format!("    {}", trimmed));
        }
        formatted.push('\n');
    }
    
    formatted
}
