# @param {Integer[][]} obstacle_grid
# @return {Integer}
def unique_paths_with_obstacles(obstacle_grid)
  height = obstacle_grid.size
  width = obstacle_grid.first.size

  pash_count = Array.new(height) { Array.new(width, 0) }
  pash_count[0][0] = obstacle_grid[0][0].zero? ? 1 : 0

  (0...height).each do |i|
    (0...width).each do |j|
      next if obstacle_grid[i][j] == 1

      if i.positive? && j.positive?
        pash_count[i][j] = pash_count[i - 1][j] + pash_count[i][j - 1]
      elsif i.positive?
        pash_count[i][j] = pash_count[i - 1][j]
      elsif j.positive?
        pash_count[i][j] = pash_count[i][j - 1]
      end
    end
  end

  pash_count.last.last
end
