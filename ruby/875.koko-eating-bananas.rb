# @param {Integer[]} piles
# @param {Integer} h
# @return {Integer}
def min_eating_speed(piles, h)
  left = 1
  right = piles.max
  current_k = right

  while left < right
    speed = ((left + right) / 2).floor

    sum_time = piles.inject(0) do |time, pile|
      time += (pile / speed.to_f).ceil
    end

    if sum_time <= h
      right = speed
      current_k = current_k <= speed ? current_k : speed
    elsif left = speed + 1
    end
  end

  current_k
end
