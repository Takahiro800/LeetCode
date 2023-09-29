# @param {String} s
# @return {Boolean}
def is_palindrome(s)
  processed_s = s.downcase.gsub(/[^a-z0-9]/, '')
  revers_s = processed_s.reverse

  processed_s == revers_s
end
