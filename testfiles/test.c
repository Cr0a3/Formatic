#include <stdio.h>

extern void call();

void callme() {
    printf("called\n");
}


// Should output:
// called
int main() {
    call();
    return 0;
}