def length_of_lis(nums)
  nums.each_with_object([]) { |x, dp| (dp[dp.bsearch_index { |y| y >= x } || dp.size] = x;) }.size
end
