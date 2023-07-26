# @param {String} s
# @param {String} t
# @return {Boolean}
def is_anagram(s, t)
  convert_to_hash(s) == convert_to_hash(t)
end

def convert_to_hash(string)
  string.split("").each_with_object(Hash.new(0)) do |char, hash|
    hash[char.to_sym] += 1
  end
end
