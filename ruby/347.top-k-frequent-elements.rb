def top_k_frequent(nums, k)
  nums.tally.max_by(k) { |_, v| v }.to_h.keys
end
