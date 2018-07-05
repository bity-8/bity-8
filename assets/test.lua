-- A basic drawing thing.
-- set the palette.
colors = {255, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255,
255, 0, 255, 255, 255, 255, 100, 0, 0, 0, 100, 0, 0, 0, 100, 0, 100, 100, 100,
100, 100, 100, 0, 100, 100, 100, 0, 50, 50, 50}

for k, v in pairs(colors) do
   _poke(0x40000+k-1, v)
end

_poke(0x40034, 0x1F) -- CHAN 1
_poke(0x40035, 0x2F) -- CHAN 1

-- _poke(0x40038, 0x2F) -- CHAN 2
-- _poke(0x40039, 0x3F) -- CHAN 2

-- _poke(0x4003C, 0x3F) -- CHAN 3
-- _poke(0x4003D, 0x2F) -- CHAN 3

-- _poke(0x40040, 0x0F) -- CHAN 4
-- _poke(0x40041, 0x1F) -- CHAN 4

function _update()
   -- set the screen.
   for i=0, 192-1 do
      _poke(0x40400 + i, math.floor(math.random() * 0x100))
   end

   for j=0, 144-1 do
      _mcpy(0x40400 + 0x60 * j, 0x40400, 0x60)
   end

   _mset(0x40400 + 0xd80, 0x1b00, 0)
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
