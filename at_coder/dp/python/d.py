# https://atcoder.jp/contests/dp/tasks/dp_d

N, W = [int(i) for i in input().split()]
ITEMS = [[int(i) for i in input().split()] for _ in range(N)]

# N, W = 3, 8
# ITEMS = [[3, 30], [4, 50], [5, 60]]

dp = [[0 for _ in range(W + 1)] for _ in range(N + 1)]

for i, item in enumerate(ITEMS):
    for w in range(W + 1):
        if item[0] <= w:
            dp[i + 1][w] = max(dp[i][w - item[0]] + item[1], dp[i][w])
        else:
            dp[i + 1][w] = dp[i][w]

print(dp[-1][-1])
