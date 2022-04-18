require 'prime'

n = 8
max = 40
vv = (2..max)
     .to_a
     .combination(n)
     .find { |arr| arr.combination(2).all?{|x, y| x.gcd(y) > 1} && arr.reduce(&:gcd) == 1 }
if vv
  print vv, ' '
  tmp = vv.map {|v| v.prime_division.map{|xi| (xi[0].to_s * xi[1])}.join}.map(&:to_i)
  p tmp
end

=begin
[6, 10, 15]                     [23, 25,      35]
[6, 10, 12, 15]                 [23, 25, 223, 35]
[6, 10, 12, 15, 18]             [23, 25, 223, 35, 233]
[6, 10, 12, 15, 18, 20]         [23, 25, 223, 35, 233, 225]
[6, 10, 12, 15, 18, 20, 24]     [23, 25, 223, 35, 233, 225, 2223]
[6, 10, 12, 15, 18, 20, 24, 30] [23, 25, 223, 35, 233, 225, 2223, 235]
=end
