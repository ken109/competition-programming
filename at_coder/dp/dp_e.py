# https://atcoder.jp/contests/dp/tasks/dp_e

N, W = [int(i) for i in input().split()]
ITEMS = [[int(i) for i in input().split()] for _ in range(N)]

# N, W = 3, 8
# ITEMS = [[3, 30], [4, 50], [5, 60]]

dp = [[float('inf') for _ in range(10 ** 5 + 1)] for _ in range(N + 1)]
dp[0][0] = 0

for i in range(1, N + 1):
    for j in range(10 ** 5 + 1):
        if j - ITEMS[i - 1][1] >= 0:
            dp[i][j] = min(dp[i - 1][j], dp[i - 1][j - ITEMS[i - 1][1]] + ITEMS[i - 1][0])
        else:
            dp[i][j] = dp[i - 1][j]

ans = 10 ** 5
while dp[N][ans] > W:
    ans -= 1

print(ans)
