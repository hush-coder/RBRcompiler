int add(int a, int b);
int add(int a, int b) {
    return a + b;
}

// 函数重复定义错误
int add(int a, int b) {
    return a - b;
}

// 函数声明与定义不匹配
int multiply(int a, int b, int c);
int multiply(int a, int b) {  // 参数数量不匹配
    return a * b;
}

int main() {
    int result = add(5, 3);
    
    // 调用参数数量不匹配的函数
    int result2 = multiply(2, 3, 4);  // 期望3个参数，但定义只有2个
    
    // 调用未定义的函数
    int result3 = undefined_func(1, 2);
    
    return result;
}
