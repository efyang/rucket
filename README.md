# rucket

rust/racket FFI library to find whether mouse coords are within a country, for use with Viviose/Risk


### Contributing

Add stuff to ```src/countrydata.txt``` in the format
```
Country Name
<rect>
<rect>
Country Name
<rect>
```

<rect> is defined as ```minx maxx miny maxy``` as in
```
 +-----+ maxy
 |     |
 +-----+ miny
minx maxx
```

Point should be based off of the included ```scaledboard.png```.
