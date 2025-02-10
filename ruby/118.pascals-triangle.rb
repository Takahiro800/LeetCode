# @param {Integer} num_rows
# @return {Integer[][]}
def generate(num_rows)
  num_rows.times.each_with_object([]) do |i, rows|
    row = Array.new(i + 1, 1)
    rows << row
    next if i.zero?

    prev_row = rows[i - 1]
    prev_row.each_with_index do |_, j|
      row[j + 1] = prev_row[j] + prev_row[j + 1] if j < prev_row.size - 1
    end
  end
end
