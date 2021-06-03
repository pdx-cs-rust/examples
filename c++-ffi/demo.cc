#include <iostream>

extern "C" {
    #include <inttypes.h>
    extern int64_t add1(int64_t *, char *);
}

using namespace std;

int main() {
    int64_t x = 3;
    int64_t y = add1(&x, (char *) "working with");
    cout << "got " << x << " " << y << endl;
    return 0;
}
