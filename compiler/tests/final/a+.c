int add(int a, int b) {
    return a + b;
}
int multiply(int a, int b) {
    return a * b;
}
int main () {
    int x = 10;
    int y = 20;
    int sum = add(x, y);
    int product = multiply(x, y);
    if(!(x>y)&& (sum > product))
    {
        return sum;
    }
    
    else
    {
        return -product;
    }
}