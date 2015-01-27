#!/usr/bin/ruby

# Main Function, recursive
# |---------------------------------------------------------------------| 71 chars
  g=->(n,m){r=[*n..m].sample;"#{g[n,r-1]if r!=n}#{g[r+1,m]if r!=m}#{r} "}

# Wrapper
#   |-----------| 13 chars
  f=->(n){g[1,n]}

# 71 + 13 = 84 chars

(1..10).each { |i| puts "n = #{i} => "+f[i]}
