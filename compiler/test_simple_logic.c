int main() {
    int a = 10;
    int b = 5;
    int result = 0;
    
    if (a > b) {
        result = result + 1;
    }
    
    if (b > 0) {
        result = result + 1;
    }
    
    if ((a > b) && (b > 0)) {
        result = result + 1;
    }
    
    return result;
}
