class MinStack
  attr_accessor :stack

  def initialize()
    @stack = []
  end


  def push(val)
    min = if stack.empty?
            val
          else
            [stack.last[1], val].min
          end
    stack << [val, min]
  end


  def pop()
    stack.pop
    return
  end


  def top()
    stack.last[0]
  end

  def get_min()
    stack.last[1]
  end
end
