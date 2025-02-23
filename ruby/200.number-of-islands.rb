# @param {Character[][]} grid
# @return {Integer}
def num_islands(grid)
  ans = 0
  grid.each_with_index do |row, i|
    row.each_with_index do |cell, j|
      next if cell != '1'

      ans += dfs(grid, i, j)
    end
  end

  ans
end

def dfs(grid, x, y)
  return 0 if x.negative? || y.negative?
  return 0 if x >= grid.size || y >= grid[0].size
  return 0 if grid[x][y] != '1'

  grid[x][y] = '0'
  dfs(grid, x - 1, y)
  dfs(grid, x + 1, y)
  dfs(grid, x, y - 1)
  dfs(grid, x, y + 1)

  1
end
