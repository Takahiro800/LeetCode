# @param {String[]} strs
# @return {String[][]}
def group_anagrams(strs)
  strs.each_with_object(Hash.new { |hash, key| hash[key] = [] }) do |str, hash|
    hash[str.chars.sort.join] << str
  end.values
end
