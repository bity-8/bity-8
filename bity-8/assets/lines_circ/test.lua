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

-- set the palette.
for k, v in pairs(colors) do
   _poke(0x40000+k-1, v)
end

t=0 m=360 c=192/2 c2=144/2 a={}
for i=0,60 do
   table.insert(a,6*i)
end

function mycos(x) return cos(x * 3.141592 * 2) end
function mysin(x) return sin(x * 3.141592 * 2) end

function _update()
   draw_rect(0, 0, 192, 144, 2)
   for i=1,61 do
      p=a[i]
      b=p/180
      r=40+20*mysin(t/m)
      d=40+20*mycos(t/m)
      draw_line(c+d*mycos(b),c2+d*mysin(b),c+r*mycos(b+.3),c2+r*mysin(b+.3),8+(t/2+p/6)%6)a[i]=(p+1)%m
   end
   t=(t+1)%m
end
