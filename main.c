#include <stdio.h>
const char* hello_from_rust_wo_std();

int main() {
        const char* str = hello_from_rust_wo_std();
        printf("%s\n", str);
}
