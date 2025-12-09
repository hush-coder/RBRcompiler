int main() {
    int x = 20;
    int y = 5;
    int result;

    result = x * y + x/y+10;

    if (result == 113 && x > y) {
        result = 1;
    } else {
        result = 0;
    }

    return result;
}

