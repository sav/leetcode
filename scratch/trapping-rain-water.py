""" 42. Trapping Rain Water
"""

def get_right_max(v):
    n = len(v)
    w = [0] * n
    max = 0
    for i in range(n-2, -1, -1):
        max = max if i+1 >= n or v[i+1] <= max else v[i+1]
        w[i] = max
    return w


def trap(v):
    right_max = get_right_max(v)

    trapped = 0
    left_max = 0

    for i in range(0, len(v)):
        if left_max > v[i] and right_max[i] > v[i]:
            height = min(left_max, right_max[i])
            trapped += height - v[i]
        if v[i] > left_max: 
            left_max = v[i]

    return trapped


print(trap([0,1,0,0,0,3]))

print(trap([0,1,0,0,0,3,0,1,0,2,0,0,3,0]))

print(trap([0,1,0,0,0,2,0,1,0,2,0,0,3,0])) # s/v[5]=3/v[5]=2