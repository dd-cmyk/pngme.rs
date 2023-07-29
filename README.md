# Simple utility for encoding messages in png files
The project is just me following the directions from the [Pngme Book](https://picklenerd.github.io/pngme_book/)

## Usage
Encode a message:
```
pngme encode <FILE_PATH> <CHUNK_TYPE> <MESSAGE> [OUTPUT_FILE]
```
Decode a message: 
```
pngme decode <FILE_PATH> <CHUNK_TYPE>
```
Remove a hidden message:
```
pngme remove <FILE_PATH> <CHUNK_TYPE>
```
Print all the chunks:
```
pngme print <FILE_PATH>
```

## Examples
Encode in the file 'dice.png' the message 'message' with with chunk type 'rust' with output file 'out.png'
```
pngme encode dice.png rust "message"
```
Decode the message in file 'out.png' hidden within the chunk with type 'rust'
```
pngme decode out.png rust
```
Print all the chunks of a file 'file.png'
```
pngme print file.png
```
Remove a chunk with a type 'rust' from a image file named 'picture.png'
```
pngme remove picture.png rust
```
