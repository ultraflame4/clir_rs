# CliR.rs
The hilarously fast successor of [CliRenderer](https://github.com/ultraflame4/CliR/) written in rust. Most of the features in the python version were kept.

<details>
  <summary><h2>Images</h2></summary>
Here is an example I rendered, using a screenshot of the old version as the image source \
<code>$ clir_rs https://github.com/ultraflame4/CliR/raw/main/resources/img.png</code>
       
![image](https://github.com/ultraflame4/clir_rs/assets/34125174/ec911c89-0dcc-47cd-b246-2d88a2268eb7)
Screenshot - https://github.com/ultraflame4/CliR/main/resources/img.png \
Original - Pixabay. (2017, February 25). View Of High Rise Buildings during Day Time

</details>


## Building & Install
> [!NOTE]  
> rust is needed to build & install this program! See [installing rust](https://www.rust-lang.org/tools/install).

Install with `cargo install --git https://github.com/ultraflame4/clir_rs`.

## Usage
```
Usage: clir_rs.exe <source> [--output <output>] [--no-autosize] [-w <width>] [-h <height>]
       [--no-keep-aspect] [--no-color] [--plain-text] [--debug] [--use-original-image-size]
       [--charset <charset>]

Renders an image to the console as unicode art

Positional Arguments:
  source            path to the source image

Options:
  --output          saves output to this path.
  --no-autosize     disables automatic resizing of output size to fit the
                    terminal if available. Using --width or --height will
                    override the detected values. When not available or
                    disabled, autosize sets width to 100, height is derived from
                    aspect ratio . If --no_keep_aspect is set, height will be
                    set to 25
  -w, --width       specify width of the output in number of chars.
  -h, --height      specify height of the output in number of rows.
  --no-keep-aspect  disables keeping of aspect ratio when resizing images. No
                    effect when both --width & --height is used.
  --no-color        disables colors, in rendered output. Result will be black &
                    white
  --plain-text      enables plain text mode, useful for rendering unicode art.
  --debug           enable debug outputs, which will be stored in
                    `./clir_rs_debug/`
  --use-original-image-size
                    overrides all size options. Uses the orginal image's size.
                    Calculation is (image.width / CELL_W, image.height / CELL_H)
                    Where CELL_W & CELL_H is typically 2 & 4 respectively.
  --charset         overrides all size options. Uses the orginal image's size.
                    Calculation is (image.width / CELL_W, image.height / CELL_H)
                    Where CELL_W & CELL_H is typically 2 & 4 respectively.
  --help            display usage information
```



## Benchmarks
Disclaimer: The benchmarks are not very scientific,as they were not done in a controlled, isloated environment, with multiple runs. \

All tests was conducted by rendering `./test_image_2.png` using a fixed size of 1000 x 500, with color. \
All results has been truncated, only showing the timings. \
The new rust version shows the timings taken for each computation section, unfortunately the old python version only prints out the total time taken.

The tests here were done on my pc which has a i7-13700k.

### CliRenderer (Python)
Command:
```shell
$ clirender ".\test_resource\test_image_2.png" -w 1000 -h 500
```

Results:
```shell
...


Final image resolution: Character Size ((1000, 328)) Image Size ((2000, 1312))
Finished in 12.508544683456421 seconds
```

### CliR.rs (Debug build)
Command:
```shell
$ ./target/debug/clir_rs.exe ".\test_resource\test_image_2.png" -w 1000 -h 500
```

Results:
```shell
...

Source Image Size (1137x747=849339) | Final Image size (2000x2000=4000000) | Cells count: 500000 (1000x500=500000)
Cell Generate Time: 203.26ms | Round Cell Pixels time: 177.95ms | String time: 396.79ms | Total compute time 778.00ms
Command completed in: 2.80s
```

### CliR.rs (Release build)
Command:
```shell
$ ./target/debug/clir_rs.exe ".\test_resource\test_image_2.png" -w 1000 -h 500
```

Results:
```shellcli
...

Source Image Size (1137x747=849339) | Final Image size (2000x2000=4000000) | Cells count: 500000 (1000x500=500000)
Cell Generate Time: 21.82ms | Round Cell Pixels time: 21.92ms | String time: 164.06ms | Total compute time 207.81ms
Command completed in: 1.08s
```
