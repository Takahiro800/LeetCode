# @param {Integer[][]} matrix
# @param {Integer} target
# @return {Boolean}
def search_matrix(matrix, target)
   m = matrix.first.length
   n = matrix.length

   left = 0
   right = m * n

   while left < right
        mid = ((left + right) / 2).floor
        num = matrix[(mid/m).floor][mid % m]

        if target == num
            return true
        end

        if target < num
            right = mid
        else
            left += 1
        end
   end

   false
end
