# https://atcoder.jp/contests/dp/tasks/dp_a

N = int(input())
H = [int(i) for i in input().split()]

# N = 4
# H = [10, 30, 40, 20]

dp = [0 for _ in range(N)]
dp[1] = abs(H[1] - H[0])

for i in range(2, N):
    dp[i] = min(
        dp[i - 1] + abs(H[i] - H[i - 1]),
        dp[i - 2] + abs(H[i] - H[i - 2])
    )

print(dp[N - 1])
