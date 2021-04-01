# https://atcoder.jp/contests/dp/tasks/dp_c

# N = int(input())
# A = [[int(i) for i in input().split()] for _ in range(N)]

N = 3
A = [[10, 40, 70], [20, 50, 80], [30, 60, 90]]

dp = [[0, 0, 0] for _ in range(N + 1)]

for i in range(N):
    for abc in [0, 1, 2]:
        dp[i + 1][abc] = max([
            dp[i][_abc] + A[i][_abc]
            for _abc in filter(lambda v: v != abc, [0, 1, 2])
        ])

print(max(dp[-1]))
