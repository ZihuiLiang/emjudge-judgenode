#include <vector>
#include <stdio.h>
using namespace std;
int main() {
    vector<int> d;
    for (int i = 0; i < 100000000; i ++) {
        d.push_back(i ^ (i - 1));
    }
    int ans = 0;
    for (int i = 0; i < 100000000; i ++) {
        ans ^= d[i];
    }
    printf("%d\n", ans);
    return 0;
}