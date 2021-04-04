N = 6
ITEMS = [(2, 3), (1, 2), (3, 6), (2, 1), (1, 3), (5, 85)]
W = 9

dp = [[0 for _ in range(W + 1)] for _ in range(N + 1)]

for i, item in enumerate(ITEMS):
    for w in range(W + 1):
        if w >= item[0]:
            dp[i + 1][w] = max(dp[i][w], dp[i][w - item[0]] + item[1])
        else:
            dp[i + 1][w] = dp[i][w]

print(dp[N][W])
