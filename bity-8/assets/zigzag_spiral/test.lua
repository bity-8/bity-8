-- ripped from pico-8 tweetcart.
-- credit goes to:
-- https://twitter.com/guerragames/status/974066713959559168

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
t = 0
function _update()
   t=t+.001
   mycls(1)
   x,y=0,0
   for i=0,1,.1 do
      b=i
      for a=0,1,.01 do
         r=96*a
         c=.03*(.6+.4*mysin(t*5))
         if a%.15<.05 then b=b+c else b=b-c end
         n=r*mycos(b+t*2)
         m=r*mysin(b+t*2)
         if a>0 then draw_line(cx+x,cy+y,cx+n,cy+m,3) end
         x,y=n,m
      end
   end
end
