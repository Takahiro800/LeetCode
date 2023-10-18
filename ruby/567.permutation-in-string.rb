# @param {String} s1
# @param {String} s2
# @return {Boolean}
def check_inclusion(s1, s2)
  hash = s1.chars.tally

  left = 0
  right = left

  while right < s2.length
    return true if hash.values.sum.zero?

    char = s2[right]
    if hash[char].to_i > 0
      hash[char] -= 1
      right += 1
    else
      hash[s2[left]] += 1 if hash.key?(s2[left])

      right += 1 if left == right
      left += 1
    end
  end
  return true if hash.values.sum.zero?

  false
end

# left < rightであるという保証が自分のコードにはない
# 存在しなかった場合、hashに何かしらの処理をしてkら left += 1するはず？
