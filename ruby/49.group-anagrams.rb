# @param {String[]} strs
# @return {String[][]}
def group_anagrams(strs)
  anagrams = Hash.new { |hash, key| hash[key] = [] }

  strs.each do |str|
    char_count = Hash.new(0)
    str.each_char { |char| char_count[char] += 1 }
    anagrams[char_count].push(str)
  end

  anagrams.values
end
