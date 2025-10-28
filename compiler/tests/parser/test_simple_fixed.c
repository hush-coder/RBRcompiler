int add(int a, int b) {
    return a + b;
}

int main() {
    int x = 10;
    int y = 5;
    int result = 0;
    
    if (x > y) {
        result = add(x, y);
    } else {
        result = x - y;
    }
    
    return result;
}
