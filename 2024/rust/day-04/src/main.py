


def traverse_tracking(final, grid, path, x, y):
    # M, M, S, S, X, X, M, A, S, M
    # M, M, S, S, X, X, M, A, S, M
    if len(path) == 4:
        print(path)
        final.append(path[:])
        return 1

    res = 0
    for dx, dy in [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)]:
        nx, ny = x + dx, y + dy
        if not (0 <= nx < len(grid) and 0 <= ny < len(grid[0])):
            continue

        # XMAS
        if grid[x][y] == "X" and grid[nx][ny] != "M":
            continue
        if grid[x][y] == "M" and grid[nx][ny] != "A":
            continue
        if grid[x][y] == "A" and grid[nx][ny] != "S":
            continue

        if (nx, ny) in path:
            continue


        path.append((nx, ny))
        res += traverse(final,grid, path, nx, ny)
        path.pop()
    return res


def traverse(grid, x, y):
    res = 0

    for dx, dy in [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)]:
        temp = ""
        for m in range(0, 4):
            nx, ny = x + dx * m, y + dy * m
            if 0 <= nx < len(grid) and 0 <= ny < len(grid[0]):
                temp += grid[nx][ny]
            else:
                break
        if temp == "XMAS":
            res += 1


    return res












def main():
    input = """
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
    """
    # Convert the input string into a list of lists
    grid = [list(line.strip()) for line in input.strip().split('\n')]


    res = 0
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            if grid[i][j] == "X":
                res += traverse(grid, i, j)

    print(res)



if __name__ == "__main__":
    main()
