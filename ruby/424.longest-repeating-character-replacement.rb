def character_replacement(s, k)
  hash = Hash.new(0)
  max = 0
  left = 0
  right = left

  while right < s.length
    char = s[right]
    hash[char] += 1

    if right - left + 1 - frequent_count(hash) > k
      hash[s[left]] -= 1
      left += 1
    end

    max = right - left + 1 if right - left + 1 > max
    right += 1
  end

  max
end

def frequent_count(hash)
  hash.values.max
end

# use std::collections::HashMap;

# fn character_replacement(s: String, k: i32) -> i32 {
#   let mut hash: HashMap<char, i32> = HashMap::new();
#   let mut max = 0;
#   let mut left = 0;
#   let mut right = left;

#   for char in s.chars() {
#     *hash.entry(char).or_insert(0) += 1;

#     if right - left + 1 - frequent_count(&hash) > k {
#       *hash.get_mut(&s.chars().nth(left).unwrap()).unwrap() -= 1;
#       left += 1;
#     }

#     max = std::cmp::max(right - left + 1, max);
#     right += 1;
#   }

#   max as i32
# }

# fn frequent_count(hash: &HashMap<char, i32>) -> i32 {
#   *hash.values().max().unwrap()
# }
