# @param {Integer[][]} obstacle_grid
# @return {Integer}
def unique_paths_with_obstacles(obstacle_grid)
  height = obstacle_grid.size
  width = obstacle_grid.first.size

  path_count = Array.new(width, 0)
  path_count[0] = obstacle_grid[0][0].zero? ? 1 : 0

  (0...height).each do |i|
    (0...width).each do |j|
      if obstacle_grid[i][j] == 1
        path_count[j] = 0
      elsif j.positive?
        path_count[j] += path_count[j - 1]
      end
    end
  end

  path_count.last
end
