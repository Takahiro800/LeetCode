# frozen_string_literal: true

BRACKET_PAIRS = {
  '(' => ')',
  '[' => ']',
  '{' => '}'
}.freeze

# @param {String} s
# @return {Boolean}
def is_valid(s)
  brackets = s.chars.each_with_object([]) do |c, stack|
    if BRACKET_PAIRS.key?(c)
      stack.push(c)
    elsif BRACKET_PAIRS[stack.pop] != c
      return false
    end
  end

  brackets.empty?
end
