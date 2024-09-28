# @param {Character[][]} grid
# @return {Integer}
def num_islands(grid)
  ans = 0
  directions = [[0, 1], [1, 0], [-1, 0], [0, -1]]

  height = grid.size
  width = grid[0].size

  grid.each_with_index do |row, i|
    row.each_with_index do |cell, j|
      if cell != '1'
        next
      end

      ans += 1

      queue = [[i, j]]

      until queue.empty?
        x, y = queue.shift
        next if grid[x][y] != '1'

        grid[y][x] = '0'

        directions.each do |dx, dy|
          new_x = x + dx
          new_y = y + dy
          next if new_x >= height || new_y >= width || new_x.negative? || new_y.negative?
          next if grid[new_x][new_y] != '1'

          queue << [new_x, new_y]
        end
      end
    end
  end

  ans
end
