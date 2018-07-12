-- ripped from pico-8 tweetcart.
-- credit goes to:
-- https://twitter.com/guerragames/status/906290557240135680
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

-- set the palette.
for k, v in pairs(colors) do
   _poke(0x40000+k-1, v)
end

function mycos(x) return cos(x * 3.141592 * 2) end
function mysin(x) return sin(x * 3.141592 * 2) end
function mycls(c) draw_rect(0, 0, 192, 144, c) end
cx, cy = 192/2, 144/2

-- cart goes here :)

a=192
r=math.random
t=0
function _update()
   t=t+r()*.01-.003
   for i=0,192 do
      x=r()*a
      y=r()*160-16
      draw_rect(x,y,r(5)-3,r(a)-cx,mysin(t+x*.004)^a*4+t*3)
   end
end
