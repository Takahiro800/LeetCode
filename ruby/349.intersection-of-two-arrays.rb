# @param {Integer[]} nums1
# @param {Integer[]} nums2
# @return {Integer[]}
def intersection(nums1, nums2)
  nums1.sort!
  nums2.sort!
  nums1.uniq!
  nums2.uniq!
  ans = []

  i = 0
  j = 0
  while i < nums1.length && j < nums2.length
    if nums1[i] == nums2[j]
      ans.push(nums1[i])
      i += 1
      j += 1
      next
    end

    if nums1[i] < nums2[j]
      i += 1
    else
      j += 1
    end
  end

  ans.to_a
end
