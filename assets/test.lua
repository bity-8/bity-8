poke(1, -10)
poke(2, 001)
poke(8, 120)
poke(10, 0x20)
poke(189, -3020)

for i=0, 200 do
   local val = peek(i)
   if val ~= 0 then
      print("val "..i.." is "..val)
   end
end
