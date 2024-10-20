# @param {Integer} m
# @param {Integer} n
# @return {Integer}
def unique_paths(m, n)
  factorial(m + n - 2) / (factorial(m - 1) * factorial(n - 1))
end

def factorial(n)
  (1..n).inject(:*)
end
