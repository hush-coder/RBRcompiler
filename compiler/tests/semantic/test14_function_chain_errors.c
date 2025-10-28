int add(int a, int b) {
    return a + b;
}

int multiply(int x, int y) {
    return x * y;
}

int main() {
    int x = 10;
    int y = 5;
    
    int result1 = add(x, y);
    int result2 = multiply(result1, 2);
    
    int result3 = add(x);
    int result4 = multiply(result3, 2, 3);
    int result5 = add(undefined_var, y);
    int result6 = undefined_func(x, y);
    
    return result2;
}
