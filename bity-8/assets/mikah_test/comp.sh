#!/bin/bash

../hex_to_bin measure.txt measure.mes
../compile_cart --sprite spritesheet.sht --tile tilemap.map --code drawing.lua --measure measure.mes --palette palette.pal out.b8
