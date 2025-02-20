def first_uniq_char(s)
  s.chars.each_with_index do |c, i|
    return i if s.index(c) == s.rindex(c)
  end

  -1
end
