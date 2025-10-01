int main() {
    int x = 42;
    int y = 10;
    int result = x + y;
    
    if (x > y) {
        while (x > 0) {
            x = x - 1;
        }
    } else {
        for (int i = 0; i < 5; i = i + 1) {
            y = y + 1;
        }
    }
    
    return result;
} 