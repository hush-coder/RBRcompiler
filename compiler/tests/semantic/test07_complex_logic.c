int main() {
    int a = 10;
    int b = 5;
    int c = 15;
    int result = 0;
    
    if ((a > b && b < c) || (a == 10 && c > 10)) {
        result = 1;
    }
    
    if (a > 0 && (b > 0 || c > 0) && !(a == b)) {
        result = 2;
    }
    
    if (a >= b && b <= c && a != c) {
        result = 3;
    }
    
    return result;
}
