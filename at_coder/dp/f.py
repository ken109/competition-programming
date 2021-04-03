# https://atcoder.jp/contests/dp/tasks/dp_f

S = input()
T = input()

# S = 'abracadabra'
# T = 'avadakedavra'

dp = [[0 for _ in range(len(T) + 1)] for _ in range(len(S) + 1)]

for i, s in enumerate(S):
    for j, t in enumerate(T):
        if s == t:
            dp[i + 1][j + 1] = dp[i][j] + 1
        else:
            dp[i + 1][j + 1] = max(dp[i][j + 1], dp[i + 1][j])

length = dp[-1][-1]
ans = ''
i, j = len(S), len(T)
while length > 0:
    if S[i - 1] == T[j - 1]:
        ans += S[i - 1]
        i -= 1
        j -= 1
        length -= 1
    elif dp[i][j] == dp[i - 1][j]:
        i -= 1
    else:
        j -= 1

print(''.join(reversed(ans)))
