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

t=0
function _update()
   mycls(8)
   t=t+.05
   for j=-8,200,8 do
      d=-8*floor(j%16/8)
      for i=d,144,16 do
         y=j+5*mycos(j/78-t)
         draw_rect(y,i,8,6,9)
      end
   end
end
