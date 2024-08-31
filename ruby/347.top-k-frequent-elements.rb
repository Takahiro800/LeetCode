def top_k_frequent(nums, k)
  hash = nums.tally
  hash.max_by(k) { |_, value| value }.to_h.keys
end
