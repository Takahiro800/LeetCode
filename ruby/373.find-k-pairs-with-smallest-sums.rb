def k_smallest_pairs(nums1, nums2, k)
  max_j = nums2.length

  nums1.each_with_object([]) do |n, heap|
    return heap if max_j.zero? && heap.length == k

    nums2.each_with_index do |m, j|
      break heap if j > max_j

      if (l = heap.bsearch_index { |pair| pair[0] + pair[1] > n + m })
        heap.insert(l, [n, m])
        heap.pop if heap.length > k
      elsif heap.length < k
        heap.push([n, m])
      else
        max_j = j
        break heap
      end
    end
  end
end
