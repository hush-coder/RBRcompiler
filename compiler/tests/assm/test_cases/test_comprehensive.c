int main() {
    int a = 10;
    int b = 5;
    int c = 3;
    
    int add_result = a + b;
    int sub_result = a - b;
    int mul_result = b * c;
    int div_result = add_result / b;
    
    int cmp1 = a > b;
    int cmp2 = b < a;
    int cmp3 = c == 3;
    
    int logic1 = cmp1 && cmp2;
    int logic2 = cmp1 || cmp3;
    int logic3 = !0;
    
    int final_result;
    if (logic1 && logic3) {
        final_result = add_result + mul_result;
    } else {
        final_result = sub_result - div_result;
    }
    
    return final_result;
}

