# @param {String[]} strs
# @return {String[][]}
def group_anagrams(strs)
  strs.group_by do |str|
    str.chars.tally
  end.map { |_, value| value }
end
