int main() {
    int a = 10;
    int b = 5;
    int c = 0;
    int result = 0;
    
    if (a && b) {
        result = result + 1;
    }
    
    if (a || c) {
        result = result + 1;
    }
    
    if (!c) {
        result = result + 1;
    }
    
    if ((a > b) && (b > 0)) {
        result = result + 1;
    }
    
    return result;
}
