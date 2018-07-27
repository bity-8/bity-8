pal(0)

draw_line(0,0,191,143,1)
draw_line(191,0,0,143,2)
draw_dot(122,132,3)
draw_dot(43,22,4)

draw_rect(0,0,8,8,0)
draw_circle(10+4,4,4,1)
draw_rect(20,0,8,8,2)
draw_circle(30+4,4,4,3)
draw_rect(40,0,8,8,4)
draw_circle(50+4,4,4,5)
draw_rect(60,0,8,8,6)
draw_circle(70+4,4,4,7)
draw_rect(80,0,8,8,8)
draw_circle(90+4,4,4,9)
draw_rect(100,0,8,8,10)
draw_circle(110+4,4,4,11)
draw_rect(120,0,8,8,12)
draw_circle(130+4,4,4,13)
draw_rect(140,0,8,8,14)
draw_circle(150+4,4,4,15)

counter=0
function _update()
    draw_rect(100,100,32,6,0)
    print(counter,100,100,1,16)
    counter = counter + 1
end