#include <stdio.h>
#include <errno.h>

int main() {
    int ch = getchar();
    if (ch == EOF) {
        if (errno != 0)
            perror("getchar");
        else
            fprintf(stderr, "getchar: end of file\n");
        return -1;
    }
    printf("%c\n", ch);
    return 0;
}
