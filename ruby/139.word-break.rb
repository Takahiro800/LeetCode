# @param {String} s
# @param {String[]} word_dict
# @return {Boolean}
def word_break(s, word_dict)
  dp = Hash.new(false)
  dp[0] = true

  word_set = Set.new(word_dict)
  (1..s.length).each do |end_i|
    (0...i).each do |start_i|
      if dp[start_i] && word_set.include?(s[start_i...end_i])
        dp[end_i] = true
        break
      end
    end
  end
  dp[s.length]
end
