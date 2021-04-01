N = 3
ITEMS = [7, 5, 3]
A = 10

dp = [[False for _ in range(A + 1)] for _ in range(N + 1)]
dp[0][0] = True

for i, item in enumerate(ITEMS):
    for a in range(A + 1):
        if a >= item:
            dp[i + 1][a] = dp[i][a] | dp[i][a - item]
        else:
            dp[i + 1][a] = dp[i][a]

print(dp[N][A])
