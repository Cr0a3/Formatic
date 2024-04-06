#include <stdio.h>

extern void call();

void callme() {
    printf("called\n");
}

extern char* value;

// Should output:
// called
// Hello World
int main() {
    call();

    printf(value);

    return 0;
}