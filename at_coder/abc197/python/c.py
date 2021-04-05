from functools import reduce

N = int(input())
A = [int(i) for i in input().split()]

m = float('inf')

for i in range(2 ** N - 1):
    ws = [False for _ in range(N)]
    for j in range(N - 1):
        if (i >> j) & 1:
            ws[j] = True
    group = []
    ors = []
    for j in range(N):
        group.append(A[j])
        if ws[j]:
            continue
        else:
            ors.append(reduce(lambda p, c: p | c, group))
            group.clear()
    xors = reduce(lambda p, c: p ^ c, ors)
    if xors < m:
        m = xors

print(m)
