#include <bits/stdc++.h>

using namespace std;

int main() {
    int N = 3;
    vector<int> A = {7, -6, 9};

    vector<int> dp(N + 1, 0);

    for (int i = 0; i < A.size(); i++) {
        dp[i + 1] = max(dp[i], dp[i] + A[i]);
    }

    cout << dp[N] << endl;

    return 0;
}
