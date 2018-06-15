-- A basic drawing thing.
-- set the palette.
colors = {255, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255,
255, 0, 255, 255, 255, 255, 100, 0, 0, 0, 100, 0, 0, 0, 100, 0, 100, 100, 100,
100, 100, 100, 0, 100, 100, 100, 0, 50, 50, 50}

for k, v in pairs(colors) do
   poke(0x40000+k-1, v)
end

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

function _update()
   -- set the screen.
   for i=0, 192 do
      for j=0, 144 do
         poke(0x40400 + j*192 + i, math.floor(math.random() * 0x100))
      end
   end
end
