#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// 交换两个元素
void swap(int* a, int* b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

// 分区函数
int partition(int arr[], int low, int high) {
    int pivot = arr[high];  // 选择最后一个元素作为基准
    int i = (low - 1);      // 小于基准的元素的索引
    
    for (int j = low; j <= high - 1; j++) {
        // 如果当前元素小于或等于基准
        if (arr[j] <= pivot) {
            i++;
            swap(&arr[i], &arr[j]);
        }
    }
    swap(&arr[i + 1], &arr[high]);
    return (i + 1);
}

// 快速排序主函数
void quicksort(int arr[], int low, int high) {
    if (low < high) {
        // 分区操作
        int pivot = partition(arr, low, high);
        
        // 递归排序左半部分
        quicksort(arr, low, pivot - 1);
        
        // 递归排序右半部分
        quicksort(arr, pivot + 1, high);
    }
}

// 打印数组
void print_array(int arr[], int size) {
    for (int i = 0; i < size; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
}

// 生成随机数组
void generate_random_array(int arr[], int size) {
    srand(time(NULL));
    for (int i = 0; i < size; i++) {
        arr[i] = rand() % 1000;
    }
}

// 验证数组是否已排序
int is_sorted(int arr[], int size) {
    for (int i = 1; i < size; i++) {
        if (arr[i] < arr[i-1]) {
            return 0;
        }
    }
    return 1;
}

int main() {
    const int ARRAY_SIZE = 100000;
    int arr[ARRAY_SIZE];
    
    // 生成随机数组
    generate_random_array(arr, ARRAY_SIZE);
    
    // 执行快速排序
    quicksort(arr, 0, ARRAY_SIZE - 1);
    
    // 验证排序结果并返回
    return is_sorted(arr, ARRAY_SIZE) ? 0 : 1;
}
