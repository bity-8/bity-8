-- ripped from pico-8 tweetcart.
-- credit goes to:
-- https://twitter.com/guerragames/status/1015430558699216896

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

t = 0
function _update()
   t=t+.1
   draw_rect(0, 0, 192, 144, 15)
   z=t/4
   for a=1,.05,-.005 do
      for i=1,14 do
         r=a*60+max(0,i*3*mysin(a+z))
         x=r*mycos(a*4-z)
         y=r*mysin(a*4-z)
         draw_circle(192/2+x,144/2+y,3,i)
      end
   end
end
