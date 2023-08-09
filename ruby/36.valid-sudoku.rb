# @param {Character[][]} board
# @return {Boolean}
def is_valid_sudoku(board)
  # SQUARE_NUM = 3
  valid_rows_and_columns?(board) && valid_boxes?(board)
end

def valid_rows_and_columns?(board)
  (0...9).each do |q|
    column = []
    row = []

    (0...9).each do |p|
      row << board[q][p]
      column << board[p][q]
    end

    return false unless valid?(row) && valid?(column)
  end

  true
end

def valid_boxes?(board)
  y_cap = 0

  until y_cap == 9
    x_cap = 0

    until x_cap == 9
      box = []
      (y_cap...(y_cap + 3)).each do |y|
        (x_cap...(x_cap + 3)).each do |x|
          box << board[y][x]
        end

        return false unless valid?(box)
      end
      x_cap += 3
    end
    y_cap += 3
  end

  true
end

def valid?(array)
  array.each_with_object({}) do |cell, hash|
    next if cell == '.'
    return false if hash.key?(cell)

    hash[cell] = true
  end

  true
end
