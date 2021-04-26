n = int(input())
av = [int(i) for i in input().split()]
bv = [int(i) for i in input().split()]

ans = min(bv) - max(av)

if 0 <= ans:
    print(ans + 1)
else:
    print(0)
