def k_smallest_pairs(nums1, nums2, k)
  heap = []
  m = nums1.size
  n = nums2.size
  ans = []

  (0...m).each do |i|
    break if heap.size >= k

    heap.push([nums1[i] + nums2[0], i, 0])
    heap.sort!
  end

  while k.positive?
    break if heap.empty?

    _sum, i, j = heap.shift
    ans.push([nums1[i], nums2[j]])

    if j + 1 < n
      heap.push([nums1[i] + nums2[j + 1], i, j + 1])
      heap.sort!
    end

    k -= 1
  end

  ans
end
