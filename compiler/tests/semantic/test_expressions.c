int main() {
    int a = 5;
    int b = 3;
    int result = 0;
    
    // 正常表达式
    result = a + b;
    result = a - b;
    result = a * b;
    result = a / b;
    result = a > b;
    result = a < b;
    result = a >= b;
    result = a <= b;
    result = a == b;
    result = a != b;
    result = -a;
    result = +b;
    result = (a + b) * (a - b);
    
    // 表达式错误：使用未定义变量
    result = c + d;  // c和d未定义
    
    // 表达式错误：赋值给未定义变量
    undefined_var = a + b;
    
    // 表达式错误：在表达式中使用未定义变量
    result = a + undefined_var2;
    
    return result;
}
