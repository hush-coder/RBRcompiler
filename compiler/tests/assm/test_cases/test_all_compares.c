int main() {
    int a = 5;
    int b = 3;
    
    int eq = (a == a);
    int ne = (a != b);
    int gt = (a > b);
    int ge = (a >= b);
    int lt = (b < a);
    int le = (b <= a);
    
    return eq + ne + gt + ge + lt + le;
}

