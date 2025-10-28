int add(int a, int b) {
    return a + b;
}

int multiply(int x, int y, int z) {
    return x * y * z;
}

int main() {
    int x = 10;
    int y = 5;
    
    int result1 = add(x);
    int result2 = add(x, y, 3);
    int result3 = multiply(x, y);
    int result4 = multiply(x, y, 3, 4);
    
    return result1;
}
