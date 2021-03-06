-- A basic drawing thing.
-- set the palette.
colors = {255, 0, 0, 255, 255, 0, 0, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255,
255, 0, 255, 255, 255, 255, 100, 0, 0, 0, 100, 0, 0, 0, 100, 0, 100, 100, 100,
100, 100, 100, 0, 100, 100, 100, 0, 50, 50, 50}

-- set the palette.
for k, v in pairs(colors) do
   _poke(0x40000+k-1, v)
end

inst = 0x1A
function _update()
   if btn(7) then play_sfx(0, 0) end
   if btn(6) then inst = math.floor(math.random() * 4) * 16 + 0xA end

   if btn(6) then pause_sfx(0)
   else resume_sfx(0) end

   -- if btn(0) then _poke(0x40035, 0x40) _poke(0x40034, inst)
   -- else _poke(0x40034, 0x00) end -- chan 1

   -- if btn(1) then _poke(0x40039, 0x42) _poke(0x40038, inst)
   -- else _poke(0x40038, 0x00) end -- chan 2

   -- if btn(2) then _poke(0x4003D, 0x44) _poke(0x4003C, inst)
   -- else _poke(0x4003C, 0x00) end -- chan 3

   -- if btn(3) then _poke(0x40041, 0x47) _poke(0x40040, inst)
   -- else _poke(0x40040, 0x00) end -- chan 4

   -- set the screen.
   for i=0, 192-1 do
      draw_line(i,0,i,144,math.floor(math.random() * 0x100))
   end

   draw_rect(0,36,192,72,3)

   -- draw font to the screen, just 'cause :P.
   for i=0, 96*72/2-1 do -- -1 because lua is inclusive
      local fnt_loc = 0x4E000 + i
      local scr_loc = 0x40400 + math.floor(i / 48) * 96 + i%48
      local val = _peek(fnt_loc)
      _poke(scr_loc, val)
   end
end
