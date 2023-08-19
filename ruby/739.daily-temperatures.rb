# @param {Integer[]} temperatures
# @return {Integer[]}
def daily_temperatures(temperatures)
  ans = []
  stack = []

  temperatures.each_with_index do |temperature, i|
    if stack.empty?
      stack.push(i)
    else
      while !stack.empty? && temperatures[stack.last] < temperature
        ans[stack.last] = i - stack.last
        stack.pop
      end
      stack.push(i)
    end
  end

  stack.each do |i|
    ans[i] = 0
  end

  ans
end
