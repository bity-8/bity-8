#!/bin/bash

../hex_to_bin measure.txt measure.mes
../compile_cart --sprite ss.sht --tile tile.map --measure measure.mes --code test.lua out.b8
