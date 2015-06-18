#!/usr/bin/ruby
# @author Tobias Münch <tobias.muench@hs-osnabrueck.de>
# Codegolf Arabic to Roman Numbers Conversion in Ruby with 131 characters.
# Thanks for Rene Helmke for some inspiration at one Point.
class RomanNumberConverter

  # Only a Wrapper
  # for testing. Not Included in counter
  def convert(x)
    c(x)
  end

  # 175 Characters
  def c_long(x)
    [1000,900,500,400,100,90,50,40,10,9,5,4,1].zip(%w[M CM D CD C XC L XL X IX V IV I]).each do |k, v|
       return  v+convert(x-k) if x>=k
    end
    ''
  end

  # 131 Characters
  def c(x)[1000,900,500,400,100,90,50,40,10,9,5,4,1].zip(%w[M CM D CD C XC L XL X IX V IV I]).each{|k,v|return v+c(x-k)if x>=k};''end
end

converter = RomanNumberConverter.new
(0..3999).each do |i|
  puts "#{i} ==> #{converter.c(i)}"
end
