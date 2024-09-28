# @param {Character[][]} grid
# @return {Integer}
def num_islands(grid)
  ans = 0
  visited = []
  directions = [[0, 1], [1, 0], [-1, 0], [0, -1]]

  height = grid.size
  width = grid[0].size

  grid.each_with_index do |row, i|
    row.each_with_index do |cell, j|
      if cell != '1' || visited.include?([i, j])
        next
      end

      ans += 1

      queue = [[i, j]]

      until queue.empty?
        x, y = queue.shift
        next if visited.include?([x, y])

        visited << [x, y]

        directions.each do |dx, dy|
          new_x = x + dx
          new_y = y + dy
          next if new_x >= height || new_y >= width || new_x.negative? || new_y.negative?
          next if grid[new_x][new_y] != '1'
          next if visited.include?([new_x, new_y])

          queue << [new_x, new_y]
        end
      end
    end
  end

  ans
end
