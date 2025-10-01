int add(int a, int b) {
    return a + b;
}

int main() {
    int x = 10;
    int y = 5;
    
    // 1. 未定义变量错误
    z = x + y;
    
    // 2. 函数调用未定义函数
    int result = undefined_func(x, y);
    
    // 3. 变量重复定义错误
    int x = 20;  // 重复定义x
    
    // 4. 函数参数数量不匹配
    int result2 = add(x);  // 缺少一个参数
    
    // 5. 使用未定义变量
    int result3 = add(undefined_var, y);
    
    return result;
}
