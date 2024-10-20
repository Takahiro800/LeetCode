# @param {Integer} m
# @param {Integer} n
# @return {Integer}
def unique_paths(m, n)
  prev_row = Array.new(n, 1)
  current_row = Array.new(n, 1)

  (1...m).each do
    (1...n).each do |j|
      current_row[j] = prev_row[j] + current_row[j - 1]
    end

    prev_row = current_row
  end

  current_row.last
end
