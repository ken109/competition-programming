N = 3
A = [7, -6, 9]

dp = [0] * (N + 1)

for i, v in enumerate(A):
    dp[i + 1] = max(dp[i], dp[i] + v)

print(dp[N])
