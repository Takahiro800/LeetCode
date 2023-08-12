# @param {String} s
# @return {Boolean}
def is_valid(s)
  hash = {
    "{": "}",
    "(": ")",
    "[": "]",
  }.freeze

  open_brackets = hash.keys.map(&:to_s)
  close_brackets = hash.values

  stack = s.chars.each_with_object([]) do |char, stack|
            if open_brackets.include?(char)
              stack.push(char)
            else
              if hash[stack.pop&.to_sym] != char
                return false
              end
            end
          end

  stack.empty?
end
