int add(int a, int b);

int add(int a, int b) {
    return a + b;
}

int main() {
    int x = 10;
    int y = 5;
    int i, j;
    int result = 0;
    
    if (x > y) {
        result = add(x, y);
    } else {
        result = x - y;
    }
    
    while (x > 0) {
        x = x - 1;
        result = result + 1;
    }
    
    for (i = 0; i < 3; i = i + 1) {
        result = result + i;
    }
    
    return result;
}
