def print_matrix(matrix):
    print('----')
    col_widths = [max(len(str(cell)) for cell in col) for col in zip(*matrix)]
    for row in matrix:
        print(" | ".join(f"{str(cell):<{col_widths[i]}}" for i, cell in enumerate(row)))


def rotate(a):
    print('----')
    n = len(a)
    for k in range(0, n//2):
        m = n-k-1
        for p in range(0, m-k):
            print(f"k={k}, m={m}, p={p}", end=" | ")
            t1 = a[k][k+p]
            t2 = a[k+p][m]
            t3 = a[m][m-p]
            t4 = a[m-p][k]
            print(f"{k},{k+p} -> {k+p},{m} -> {m},{m-p} -> {m-p},{k}",
                  end=" | ")
            print(f"{t1} -> {t2} -> {t3} -> {t4}")
            a[k][k+p] = t4
            a[k+p][m] = t1
            a[m][m-p] = t2
            a[m-p][k] = t3
    return a


a = [[1, 2, 3, 4],
     [5, 6, 7, 8],
     [9, 10, 11, 12],
     [13, 14, 15, 16]]

print_matrix(a)
a = rotate(a)
print_matrix(a)

a = [[1, 2, 3, 4, 5],
     [6, 7, 8, 9, 10],
     [11, 12, 13, 14, 15],
     [16, 17, 18, 19, 20],
     [21, 22, 23, 24, 25]]

print_matrix(a)
a = rotate(a)
print_matrix(a)

a = [[1, 2, 3, 4, 5, 6],
     [7, 8, 9, 10, 11, 12],
     [13, 14, 15, 16, 17, 18],
     [19, 20, 21, 22, 23, 24],
     [25, 26, 27, 28, 29, 30],
     [31, 32, 33, 34, 35, 36]]

print_matrix(a)
a = rotate(a)
print_matrix(a)
