# @param {Integer[]} prices
# @return {Integer}
def max_profit(prices)
  buy_price = prices[0]
  max_profit = 0

  prices.each do |price|
    if price < buy_price
      buy_price = price
    elsif price - buy_price > max_profit
      max_profit = price - buy_price
    end
  end

  max_profit
end
