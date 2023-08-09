# @param {Character[][]} board
# @return {Boolean}
def is_valid_sudoku(board)
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

def extract_box(board, y, x)
  box = []
  (y...(y+3)). each do |j|
    (x...(x+3)). each do |i|
      box << board[j][i]
    end
  end

  box
end

def valid_boxes?(board)
  start_points = (0..2).flat_map { |y| (0..2).map { |x| [y*3, x*3] } }

  box_validities = start_points.map do |y, x|
    box = extract_box(board, y, x)
    valid?(box)
  end

  box_validities.all?
end

def valid?(array)
  array.each_with_object({}) do |cell, hash|
    next if cell == '.'
    return false if hash.key?(cell)

    hash[cell] = true
  end

  true
end
