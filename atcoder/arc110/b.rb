if line = ARGV[0]
  n = line.to_i
else
  n = 4
end
rs = [];
[1, 0].repeated_permutation(n).each do |arr|
  rs.push "#{n} #{arr.join}: " + `echo #{n} #{arr.join} | ../target/debug/b`.split(/\R/).last
end
puts rs
