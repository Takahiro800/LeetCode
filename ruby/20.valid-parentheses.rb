BRACKET_PAIRS = {
  '(' => ')',
  '[' => ']',
  '{' => '}'
}.freeze

# @param {String} s
# @return {Boolean}
def is_valid(s)
  brackets = s.chars.each_with_object([]) do |char, stack|
    if BRACKET_PAIRS.key?(char)
      stack.push(char)
    elsif BRACKET_PAIRS[stack.pop] != char
      return false
    end
  end

  brackets.is_empty?
end
