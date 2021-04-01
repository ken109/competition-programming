N = 5
ITEMS = [7, 5, 3, 1, 8]
A = 12

MOD = 1000000009

dp = [[0 for _ in range(A + 1)] for _ in range(N + 1)]
dp[0][0] = 1

for i, item in enumerate(ITEMS):
    for a in range(A + 1):
        if a >= item:
            dp[i + 1][a] = (dp[i][a] + dp[i][a - item]) % MOD
        else:
            dp[i + 1][a] = dp[i][a] % MOD

print(dp[N][A])
