#include <stdio.h>

#include <inttypes.h>
int64_t add1(int64_t *x, char * msg) {
    *x += 1;
    printf("%s %ld\n", msg, *x);
    return *x;
}
