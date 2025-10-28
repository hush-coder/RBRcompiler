#include <stdlib.h>
#include <time.h>
#include <stdio.h>

int partition(int arr[], int low, int high) {
    int pivot = arr[high];
    int i = low - 1;
    
    for (int j = low; j < high; j++) {
        if (arr[j] <= pivot) {
            i++;
            int temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
        }
    }
    
    int temp = arr[i + 1];
    arr[i + 1] = arr[high];
    arr[high] = temp;
    
    return i + 1;
}

void generate_array(int arr[], int size) {
    srand(42);
    for (int i = 0; i < size; i++) {
        arr[i] = rand() % 10000;
    }
}

int main() {
    const int SIZE = 50000;
    int arr[SIZE];
    
    generate_array(arr, SIZE);
    
    clock_t total_time = 0;
    int sum = 0;
    
    for (int k = 0; k < 100; k++) {
        generate_array(arr, SIZE);
        
        clock_t start = clock();
        int pivot = partition(arr, 0, SIZE - 1);
        clock_t end = clock();
        
        total_time += (end - start);
        sum += pivot;
    }
    
    double avg_time_ms = ((double)total_time / CLOCKS_PER_SEC * 1000) / 100;
    printf("分区函数平均执行时间: %.3f 毫秒\n", avg_time_ms);
    printf("总共执行: 100 次\n");
    
    return sum % 256;
}

