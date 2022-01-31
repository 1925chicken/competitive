dp = [[0 for i in range(4)] for j in range(4)]
f = list(map(int,input().split()))
for i in range(4):
    dp[0][i] = f[i]
for i in range(3):
    for j in range(4):
        if j - 1 >= 0:
            dp[i + 1][j] += dp[i][j - 1]
        if j + 1 < 4:
            dp[i + 1][j] += dp[i][j + 1]
        dp[i + 1][j] += dp[i][j]
print(dp[3][3])
