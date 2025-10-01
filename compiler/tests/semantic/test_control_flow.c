int main() {
    int x = 10;
    int y = 5;
    
    // 正常控制流
    if (x > y) {
        x = x - 1;
    } else {
        x = x + 1;
    }
    
    while (x > 0) {
        x = x - 1;
    }
    
    for (int i = 0; i < 5; i = i + 1) {
        y = y + 1;
    }
    
    // 控制流错误：条件表达式中使用未定义变量
    if (z > 0) {  // z未定义
        x = 1;
    }
    
    // 控制流错误：while条件中使用未定义变量
    while (undefined_var > 0) {  // undefined_var未定义
        x = x - 1;
    }
    
    // 控制流错误：for循环中使用未定义变量
    for (int j = 0; k < 5; j = j + 1) {  // k未定义
        y = y + 1;
    }
    
    // 控制流错误：在控制流中使用未定义变量
    if (x > y) {
        undefined_var2 = 10;  // 在if块中使用未定义变量
    }
    
    return x + y;
}
