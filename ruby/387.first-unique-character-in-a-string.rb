def first_uniq_char(s)
  s.each_char do |c|
    index = s.index(c)
    r_index = s.rindex(c)
    return index if index == r_index
  end

  -1
end
