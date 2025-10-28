fn main() {
    // 生成 LALRPOP 语法文件
    lalrpop::process_root().unwrap();
}
