class MinStack
  attr_accessor :stack, :min

  def initialize()
    @stack = []
    @min = []
  end


  def push(val)
    if min.empty? || val <= min.last
      min << val
    end

    stack << val
  end


  def pop()
    if stack.last == min.last
      min.pop
    end

    stack.pop
    return
  end


  def top()
    stack.last
  end

  def get_min()
    min.last
  end
end
