# @param {String} s
# @return {Boolean}
def is_palindrome(s)
  processed_s = s.downcase.gsub(/[^a-z0-9]/, '')
  processed_s == processed_s.reverse
end
