pal(0)

spritex = 13
spritey = 31
tilex = 0 tiley = 0
load_sprites(0,0)
load_tilemap(0,0)
shouldDraw = true;

function _update()
   -- set the screen.
  if shouldDraw then
    tilemap(0,0,tilex,tiley)
    sprite_t(0,4,2,spritex,spritey,15)
    print("Welcome to BITY-8! °", 40, 10, 0, 16)
    print(": Move", 66, 17, 0, 16)
    print("X: Run", 80, 24, 0, 16)
    print("Z: Play song", 60, 31, 0, 16)
    print(spritex..", "..spritey, 144, 128, 0, 16)
    print(tilex..", "..tiley, 144, 122, 0, 16)
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
    if spritex <= -1 then
      if tilex <= 0 then
        spritex = -1
        shouldDraw = false;
      else
        spritex = -1
        tilex = tilex - 1
      end
    end
  elseif btn(1) then
    spritex = spritex + step
    shouldDraw = true
    if spritex >= 185 then
      if tilex < 576 then
        tilex = tilex + 1
        spritex = 185
      else
        spritex = 185
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

  if btn(4) then
    play_sfx(0, 0)
  end
end
