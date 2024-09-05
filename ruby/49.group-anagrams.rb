# @param {String[]} strs
# @return {String[][]}
def group_anagrams(strs)
  strs.sort.gruoup_by { |str| str.chars.sort }.values
end
