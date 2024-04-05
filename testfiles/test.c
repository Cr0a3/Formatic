#include <stdio.h>

extern void call();

void callme() {
    printf("called\n");
}

// Should output:
// called
// Hello World
void main() {
    call();
    printf("Hello World\n");
}