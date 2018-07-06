-- A basic drawing thing.
-- set the palette.
colors = {255, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255,
255, 0, 255, 255, 255, 255, 100, 0, 0, 0, 100, 0, 0, 0, 100, 0, 100, 100, 100,
100, 100, 100, 0, 100, 100, 100, 0, 50, 50, 50}

for k, v in pairs(colors) do
   _poke(0x40000+k-1, v)
end

rotx = 0;
roty = 96;
delay = 0;
circle = true;
function _update()
   -- set the screen.
   for i=0, 192-1 do
      _poke(0x40400 + i, 0x99)--math.floor(math.random() * 0x100))
   end

   for j=0, 144-1 do
      _mcpy(0x40400 + 0x60 * j, 0x40400, 0x60)
   end

   --_mset(0x40400 + 0xd80, 0x1b00, 0)
   draw_line(0,0,192,144,0)
   draw_line(0,143,192,0,3)
   draw_line(21,40,30,40,10)
   if circle and delay <= 60 then
      for i=0, 20 do
         x = math.random(192) 
         y = math.random(144)
         r = math.random(30)
         c = math.random(16)
         draw_circle(x,y,r,c)
         circle = false
      end
   elseif delay <= 60 then
      for i=0, 50 do
         draw_rect(math.random(192), math.random(144), math.random(192), math.random(144), math.random(16))
         circle = true
      end
   else
      delay = 0
   end
   delay = delay + 1
   --draw_circle(127, 144, 29, 6)
   draw_line(96,72,rotx,roty,5)
   if rotx <= 0 then
      if roty <= 0 then
         rotx = 1
      else
         roty = roty - 11
      end
   elseif rotx >= 192 then
      if roty >= 144 then
         rotx = 191
      else
         roty = roty + 11
      end
   elseif roty <= 0 then
      rotx = rotx + 11
   elseif roty >= 144 then
      rotx = rotx - 11
   end
end

-- The below test should print NOTHING, because this is all in the read-only
-- section of the cartridge.
_poke(2, 001)
_poke(8, 120)
_poke(10, 0x20)
_poke(189, -3020)

for i=0, 200 do
   local val = _peek(i)
   if val ~= 0 then
      print("val "..i.." is "..val)
   end
end
