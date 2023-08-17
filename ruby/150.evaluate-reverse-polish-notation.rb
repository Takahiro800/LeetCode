# @param {String[]} tokens
# @return {Integer}
def eval_rpn(tokens)
  operators = %w(+ - * /).freeze

  tokens.each_with_object([]) do |char, stack|
    if operators.include?(char)
      arg2 = stack.pop
      arg1 = stack.pop
      stack.push(do_calculation(char, arg1, arg2))
    else
      stack.push(char.to_i)
    end
  end.first
end

def do_calculation(operand, a, b)
  case operand
  when "+"
    return a + b
  when "-"
    return a - b
  when "*"
    return a * b
  when "/"
    return (a.to_f / b).to_i
  end
end
