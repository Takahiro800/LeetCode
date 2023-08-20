# @param {Integer[]} temperatures
# @return {Integer[]}
def daily_temperatures(temperatures)
  ans = Array.new(temperatures.length, 0)
  stack = []

  temperatures.each_with_index do |temp, i|
    while !stack.empty? && temperatures[stack.last] < temp
      ans[stack.last] = i - stack.last
      stack.pop
    end

    stack.push(i)
  end

  ans
end
