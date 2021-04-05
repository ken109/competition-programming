H, W, X, Y = [int(i) for i in input().split()]

table = []

for h in range(H):
    table.append([i == '#' for i in input()])

b_pass = False
h_count = 0
for h in range(H):
    if not b_pass and table[h][Y - 1]:
        h_count = 0
    elif b_pass and table[h][Y - 1]:
        break
    elif not table[h][Y - 1]:
        h_count += 1

    if h == X - 1:
        b_pass = True

b_pass = False
w_count = 0
for w in range(W):
    if not b_pass and table[X - 1][w]:
        w_count = 0
    elif b_pass and table[X - 1][w]:
        break
    elif not table[X - 1][w]:
        w_count += 1

    if w == Y - 1:
        b_pass = True

print(h_count + w_count - 1)
