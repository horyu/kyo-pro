n = 5
rs = []
(0..(n - 2)).to_a.permutation(n - 1).each do |orders|
  arr = (0...n).to_a.reverse
  orders.each do |o|
    arr[o], arr[o + 1] = arr[o + 1], arr[o]
  end
  rs << [
    arr.join,
    arr.map.with_index{|x, i| n - 1 - (x - i).abs}.join,
    arr.map.with_index{|x, i| arr[0..i].filter{|y| y < x}.size}.join,
    orders.reverse.join,
  ].join(?\t)
end
puts rs.sort
