# @param {Integer[]} heights
# @return {Integer}
def largest_rectangle_area(heights)
  current_height = 1
  temp_max = 0

  while heights.any? { |h| h.positive? }
    width = 0
    max_width = 0

    heights.each do |h|
      if h.positive?
        width += 1
      else
        max_width = [max_width, width].max
        width = 0
      end
    end
    max_width = width if width > max_width

    temp_max = [temp_max, current_height * max_width].max
    heights.map! { |h| h -= 1}
    current_height += 1
  end

  temp_max
end
