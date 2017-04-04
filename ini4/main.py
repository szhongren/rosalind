def sumOdds(a, b):
    if a % 2 == 0:
        return sumOdds(a + 1, b)
    if b % 2 == 0:
        return sumOdds(a, b - 1)
    num = (b - a) // 2 + 1
    return num * (a + b) // 2

print(sumOdds(100, 200))
print(sumOdds(4448, 9329))
