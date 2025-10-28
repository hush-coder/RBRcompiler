int main() {
    int a = 10;
    int b = 5;
    int c = 15;
    int result = 0;
    
    if (a > b && b < c) {
        result = 1;
    }
    
    if (a < b || b < c) {
        result = 2;
    }
    
    if (!(a < b)) {
        result = 3;
    }
    
    int x = -a;
    int y = +b;
    int z = -c;
    
    if (x < 0 && y > 0 && z < 0) {
        result = 4;
    }
    
    return result;
}
