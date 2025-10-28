

int main() {
    int x = 10;
    int x = 20;
    
    int y = 30;
    
    if (x > 5) {
        int z = 40;
        int sum1 = x + y + z;
    }
    
    int sum2 = z;
    
    int result1 = add(x, y);
    int result2 = global_func(x, y, z);
    
    int result3 = undefined_var;
    
    return result1;
}

int global_func(int x, int y) {
    return x + y;
}
