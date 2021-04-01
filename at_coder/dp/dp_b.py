# https://atcoder.jp/contests/dp/tasks/dp_b

# N, K = [int(i) for i in input().split()]
# H = [int(i) for i in input().split()]

N, K = 10, 4
H = [40, 10, 20, 70, 80, 10, 20, 70, 80, 60]

dp = [0 for _ in range(N)]

for i in range(1, N):
    dp[i] = min([
        dp[i - j] + abs(H[i] - H[i - j])
        for j in range(1, K + 1 if i > K else i + 1)
    ])

print(dp[N - 1])
