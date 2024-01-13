# frozen_string_literal: true

$dp = {}

def score(dots, blocks, dots_index, blocks_index, broken_length)
  key = [dots_index, blocks_index, broken_length]

  if $dp[key]
    return $dp[key]
  end

  if dots_index == dots.length
    if blocks_index == blocks.length && broken_length == 0
      return 1
    elsif blocks_index == blocks.length - 1 && blocks[blocks_index] == broken_length
      return 1
    else
      return 0
    end
  end

  answer = 0

  dot = dots[dots_index]

  for symbol in ['.', '#']
    if dot == symbol || dot == '?'
      if symbol == '.' && broken_length == 0
        answer += score(dots, blocks, dots_index + 1, blocks_index, 0)
      elsif symbol == '.' && broken_length > 0 && blocks_index < blocks.length && blocks[blocks_index] == broken_length
        answer += score(dots, blocks, dots_index + 1, blocks_index + 1, 0)
      elsif symbol == '#'
        answer += score(dots, blocks, dots_index + 1, blocks_index, broken_length + 1)
      end
    end
  end

  $dp[key] = answer

  answer
end

def valid?(dots, blocks)
  broken_count = 0
  seen = []
  dots.each_char do |dot|
    if dot == '.'
      seen.append(broken_count) if broken_count.positive?
      broken_count = 0
    elsif dot == '#'
      broken_count += 1
    end
  end
  seen.append(broken_count) if broken_count.positive?

  seen == blocks
end

def parse(line)
  dots, blocks = line.split
  blocks = blocks.split(',').map(&:to_i)
  [dots, blocks]
end

def pt1(filename)
  lines = File.open(filename).readlines.map(&:chomp)

  answer = 0

  lines.each do |line|
    dots, blocks = parse(line)

    $dp = {}

    answer += score(dots, blocks, 0, 0, 0)
  end

  answer
end

def pt2(filename)
  lines = File.open(filename).readlines.map(&:chomp)

  answer = 0

  lines.each do |line|
    dots, blocks = parse(line)

    dots = Array.new(5, dots).join('?')

    blocks *= 5

    $dp = {}

    answer += score(dots, blocks, 0, 0, 0)
  end

  answer
end

puts(pt1('12_example.txt'))
puts(pt1('12.txt'))

puts(pt2('12_example.txt'))
puts(pt2('12.txt'))
