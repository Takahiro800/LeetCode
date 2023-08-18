# @param {Integer} n
# @return {String[]}
def generate_parenthesis(n)
  parenthesis = []
  recurse(n, "", parenthesis, 0, 0)
  parenthesis
end

def recurse(n, pre, parenthesis, opens, closes)
  if opens + closes == n * 2
    parenthesis << pre
    return
  end

  if closes < opens
    recurse(n, pre+")", parenthesis, opens, closes+1)
  end

  if opens < n
    recurse(n, pre+"(", parenthesis, opens+1, closes)
  end
end
