int add(int a, int b) {
    return a + b;
}

int sub(int a, int b) {
    return a - b;
}

int calc(int x, int y) {
    int sum = add(x, y);
    int diff = sub(x, y);
    return sum + diff;
}

int main() {
    int result = calc(10, 3);
    return result;
}

