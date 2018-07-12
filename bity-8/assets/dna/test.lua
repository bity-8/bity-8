-- ripped from pico-8 tweetcart.
-- credit goes to:
-- https://twitter.com/jefrsilva/status/968646725626925056
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
p=0
c={8,9,10,11,12,1,2}
function _update()
   mycls(0)
   for i=0,192 do
      s=cy+cy/2*mysin((i+p)/100)
      t=c[i%7+1]
      z=64+(i-64)/2
      draw_circle(z,s,2,t)
      draw_line(z,s,i,s,t)
      draw_circle(i,s,2,t)
   end
   p=p+1
end
