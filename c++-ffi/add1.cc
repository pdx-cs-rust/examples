#include <iostream>

using namespace std;

extern "C" {
    #include <inttypes.h>
    int64_t add1(int64_t *x, char * msg) {
        *x += 1;
        cout << msg << " " << *x << endl;
        return *x;
    }
}
