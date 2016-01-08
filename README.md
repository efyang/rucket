# rucket

rust/racket FFI library to find whether mouse coords are within a country, for use with [Viviose/Risk](https://github.com/Viviose/Risk)


### Contributing

Add stuff to ```src/countrydata.txt``` in the format
```
Country Name
<rect>
<rect>
Country Name
<rect>
```

<rect> is defined as ```minx miny width height``` as in
``` minx  
miny +-----+
     |     | height
     +-----+
      width
```
(gimp compatible)

Point should be based off of the included ```scaledboard.png```.

Rust code is generated into ```lib.rs``` from ```base.rs``` and the parsed ```countrydata.txt```.
