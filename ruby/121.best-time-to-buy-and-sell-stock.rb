# @param {Integer[]} prices
# @return {Integer}
def max_profit(prices)
  left = 0
  right = left + 1

  current_max = 0

  while right < prices.length
    diff = prices[right] - prices[left]
    if diff > 0
      current_max = diff if diff > current_max
      right += 1
    else
      left = right
      right = left + 1
    end
  end

  current_max
end
