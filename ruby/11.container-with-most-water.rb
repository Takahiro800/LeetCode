# @param {Integer[]} height
# @return {Integer}
def max_area(height)
  l = 0
  r = height.size - 1
  res = 0

  while l < r
    if height[l] < height[r]
      s = height[l] * (r - l)
      l += 1
    else
      s = height[r] * (r - l)
      r -= 1
    end
    res = [res, s].max
  end

  res
end
