#include <stdio.h>

int *f() {
    int y = 7;
    return &y;
}

int main() {
    int *yp = f();
    printf("%d\n", *yp);
}
