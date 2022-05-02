h = {}
base = "123456789"
h[base] = ""
loop do
  new_h = h.clone
  h.each do |k, v|
    rotate = k[1..] + k[0]
    if !h[rotate]
      new_h[rotate] = v + "0"
    end
    rev = k.reverse
    if !h[rev]
      new_h[rev] = v + "1"
    end
  end
  if new_h.count == h.count
    break;
  end
  h = new_h
end
# p h
h.delete base
h.each do |_, v|
  k = base
  v.chars.reverse_each do |c|
    if c == "0"
      k = k[-1] + k[0...-1]
    else
      k = k.reverse
    end
  end

  x = k.chars.each_cons(2).map{|ab| ab[0] < ab[1]}.chunk{|tf| tf}.map{|tf, arr| (tf ? ?T : ?F) + arr.count.to_s }.join(" ")
  p [k, v, x]
end
