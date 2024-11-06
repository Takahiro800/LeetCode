# @param {String} s
# @param {String[]} word_dict
# @return {Boolean}
def word_break(s, word_dict)
  return false if word_dict.empty?

  dp = Array.new(s.size + 1, false)
  dp[0] = true

  (1..s.size).each do |i|
    word_dict.select { |word| word.size <= i }.each do |word|
      dp[i] ||= dp[i - word.size] && s[i - word.size...i] == word
    end
  end

  dp.last
end
