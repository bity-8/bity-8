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

spritex = 13 spritey = 31
tilex = 0 tiley = 0
load_sprites(0,0)
load_tilemap(0,0)
shouldDraw = true;
function _update()
   -- set the screen.
  if shouldDraw then
    tilemap(0,0,tilex,tiley)
    sprite_t(0,4,2,spritex,spritey,15)
    print("Welcome to BITY-8! °", 10, 17, 1, 16)
    print(spritex..", "..spritey, 10, 10, 1, 16)
    shouldDraw = false;
  end
  if btn(5) then
    step = 2
  else
    step = 1
  end
  if btn(0) then
    spritex = spritex - step
    shouldDraw = true;
    if spritex < -7 then
      if tilex <= 0 then
        spritex = -7
        shouldDraw = false;
      else
        spritex = 191
        tilex = tilex - 192
      end
    end
  elseif btn(1) then
    spritex = spritex + step
    shouldDraw = true
    if spritex > 191 then
      if tilex < 576 then
        tilex = tilex + 192
        spritex = -7
      else
        spritex = 191
        shouldDraw = false;
      end
    end
  end
  if btn(2) then
    spritey = spritey - step
    shouldDraw = true
    if spritey < -7 then
      if tiley <= 0 then
        spritey = -7
        shouldDraw = false;
      else
        spritey = 143
        tiley = tiley - 144
      end
    end
  elseif btn(3) then
    spritey = spritey + step
    shouldDraw = true
    if spritey > 143 then
      if tiley < 432 then
        spritey = -7
        tiley = tiley + 144
      else
        spritey = 143
        shouldDraw = false
      end
    end
  end
end
