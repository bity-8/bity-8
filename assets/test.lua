-- A basic drawing thing.
-- set the palette.
colors = {255, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255,
255, 0, 255, 255, 255, 255, 100, 0, 0, 0, 100, 0, 0, 0, 100, 0, 100, 100, 100,
100, 100, 100, 0, 100, 100, 100, 0, 50, 50, 50}

-- set the palette.
for k, v in pairs(colors) do
   _poke(0x40000+k-1, v)
end

-- thanks to https://stackoverflow.com/questions/5977654/lua-bitwise-logical-operations
function BitAND(a,b)--Bitwise and
    local p,c=1,0
    while a>0 and b>0 do
        local ra,rb=a%2,b%2
        if ra+rb>1 then c=c+p end
        a,b,p=(a-ra)/2,(b-rb)/2,p*2
    end
    return c
end

inst = 0x1A
loop_ind = 0
function _update()
   inp = _peek(0x40031)

   loop_ind = loop_ind + 1
   if BitAND(inp, 16) > 0 then
      inst = loop_ind % 4 * 16 + 0xA
   end

   if BitAND(inp, 1) > 0 then _poke(0x40035, 0x40) _poke(0x40034, inst)
   else _poke(0x40034, 0x00) end -- chan 1

   if BitAND(inp, 2) > 0 then _poke(0x40039, 0x42) _poke(0x40038, inst)
   else _poke(0x40038, 0x00) end -- chan 2

   if BitAND(inp, 4) > 0 then _poke(0x4003D, 0x44) _poke(0x4003C, inst)
   else _poke(0x4003C, 0x00) end -- chan 3

   if BitAND(inp, 8) > 0 then _poke(0x40041, 0x47) _poke(0x40040, inst)
   else _poke(0x40040, 0x00) end -- chan 4

   -- set the screen.
   for i=0, 192-1 do
      _poke(0x40400 + i, math.floor(math.random() * 0x100))
   end

   for j=0, 144-1 do
      _mcpy(0x40400 + 0x60 * j, 0x40400, 0x60)
   end

   _mset(0x40400 + 0xd80, 0x1b00, 0)
end
