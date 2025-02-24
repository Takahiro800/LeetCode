# frozen_string_literal: true

DIRECTIONS = [[0, 1], [1, 0], [-1, 0], [0, -1]].freeze

# @param {Integer[][]} grid
# @return {Integer}
def max_area_of_island(grid)
  height = grid.size
  width = grid[0].size
  ans = 0
  visited = Set.new

  grid.each_with_index do |row, i|
    row.each_with_index do |cell, j|
      next if cell.zero? || visited.include?([i, j])

      area = 0
      iland = [[i, j]]

      until iland.empty?
        y, x = iland.shift
        next if visited.include?([y, x])

        area += 1
        visited.add([y, x])

        DIRECTIONS.each do |dy, dx|
          new_x = x + dx
          new_y = y + dy

          next if new_x.negative? || new_x >= width
          next if new_y.negative? || new_y >= height
          next if grid[new_y][new_x] != 1

          iland.push([new_y, new_x])
        end

      end
      ans = [ans, area].max
    end
  end

  ans
end
