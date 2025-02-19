
txt_test = """
2 54 992917 5270417 2514 28561 0 990
"""

def process_two(txt):
    # Read input from file
    res = 0
    nums = [int(x) for x in txt.strip().split(' ')]
    for n in nums:
        res += ans(n, 75)

    return res

cache = {}
def ans(x, n):
    if n == 0:
        return 1

    if (x, n) not in cache:
        if x == 0:
            res = ans(1, n - 1)
        elif len(str(x)) %2 == 0:
            xlen = len(str(x))
            res = ans(int(str(x)[:xlen//2]), n-1) + ans(int(str(x)[xlen//2:]), n-1)
        else:
            res = ans(x*2024, n-1)

        cache[(x, n)] = res

    return cache[(x, n)]


import os
# Get the directory containing the script
script_dir = os.path.dirname(os.path.abspath(__file__))
# Go up one level to the day-05 directory
input_path = os.path.join(script_dir, "..", "input2.txt")

if __name__ == "__main__":
    with open(input_path) as f:
        txt = f.read()
        print(process_two(txt))
