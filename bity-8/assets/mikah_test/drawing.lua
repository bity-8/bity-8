-- A basic drawing thing.
-- set the palette.
colors = {255, 255, 255,
          0, 0, 0,
          168, 0, 0,
          0, 168, 0,
          0, 0, 168,
          115, 0, 168,
          168, 102, 0,
          230, 148, 163,
          255, 161, 0,
          255, 239, 0,
          86, 86, 86,
          169, 169, 169,
          0, 210, 224,
          0, 20, 98,
          44, 87, 39,
          255, 0, 255}

for k, v in pairs(colors) do
   _poke(0x40000+k-1, v)
end

rotx = 0;
roty = 96;
delay = 0;
circle = true;
spritex = 13 spritey = 31
dummy()
function _update()
   -- set the screen.
   for i=0, 192-1 do
      _poke(0x40400 + i, 0x11)--math.floor(math.random() * 0x100))
   end

   for j=0, 144-1 do
      _mcpy(0x40400 + 0x60 * j, 0x40400, 0x60)
   end

   --_mset(0x40400 + 0xd80, 0x1b00, 0)
   draw_rect(0,0,2,4,0)
   draw_rect(2,0,2,4,1)
   draw_rect(4,0,2,4,2)
   draw_rect(6,0,2,4,3)
   draw_rect(8,0,2,4,4)
   draw_rect(10,0,2,4,5)
   draw_rect(12,0,2,4,6)
   draw_rect(14,0,2,4,7)
   draw_line(0,0,192,144,0)
   draw_line(0,143,192,0,3)
   --draw_line(21,40,30,40,10)
   spritesheet()
   sprite_t(0,4,2,spritex,spritey,15)
  if btn(5) then
    step = 2
  else
    step = 1
  end
  if btn(0) then
    spritex = spritex - step
    if spritex < -7 then
      spritex = 191
    end
  elseif btn(1) then
    spritex = spritex + step
    if spritex > 191 then
      spritex = -7
    end
  end
  if btn(2) then
    spritey = spritey - step
    if spritey < -7 then
      spritey = 143
    end
  elseif btn(3) then
    spritey = spritey + step
    if spritey > 143 then
      spritey = -7
    end
  end
   if circle and delay <= 60 then
      for i=0, 20 do
         x = math.random(192) 
         y = math.random(144)
         r = math.random(30)
         c = math.random(16)
   --      draw_circle(x,y,r,c)
         circle = false
      end
   elseif delay <= 60 then
      for i=0, 50 do
   --      draw_rect(math.random(192), math.random(144), math.random(192), math.random(144), math.random(16))
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
