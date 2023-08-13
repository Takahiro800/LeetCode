class MinStack
  attr_accessor :stack, :minimum

  def initialize()
    @stack = []
    @minimum = nil
  end


  def push(val)
    self.minimum = minimum.nil? ? val : [minimum, val].min
    self.stack = [val, stack].flatten

    return
  end


  def pop()
    new_stack = stack[1..]

    @stack = new_stack
    @minimum = new_stack.min
    "minimum: #{minimum}"

    return
  end


  def top()
    stack[0]
  end

  def get_min()
    minimum
  end
end
