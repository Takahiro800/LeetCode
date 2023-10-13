# @param {String} s
# @return {Integer}
def length_of_longest_substring(s)
  max = 0
  appear = {}
  window = []

  arr = s.chars
  left = 0
  right = 0

  arr.each_with_index do |char, right|
    while appear.has_key?(char)
      appear.delete(arr[left])
      left += 1
    end
    appear[char] = right
    max = [max, right - left + 1].max
  end

  max
end
