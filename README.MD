# MD_Generator

## About

MD_Generator is a small tool (for Linux and Mac Os) to create a md file with the contents of C/C++ code programmed by students at Saxion. Given that to upload on the school system students usually create a MD file, fill it manually and then convert it to PDF, I resolved to automate the filling a MD file manually part.

## Installing

To install it, you first need ```cargo``` and a Rust toolchain installed. For this, visit [rust-lang.org](https://rust-lang.org) and follow the instructions on the download section. After cloning the repository, run ```cargo install --path $PATH_TO_REPO```. You should then be able to execute md_generator on your computer.

## Usage

To use it, just run ```md_generator -n "$YOUR_NAME" -c $YOUR_CLASS -s $STUDENT_NUMBER -w $QUARTER_WEEK -a "$PATH_TO_FILE1 $PATH_TO_FILE2 $PATH_TO_FILE3 $PATH_TO_FILE_N"```. You can specify multiple assignments, which will be numbered from 1 to n (n being the number of assignments), which will be written on the md file according to the order you pass the arguments. To do so, simply do ```-a``` again followed by the path of the files. Note that your full name must be within double quotes as it has spaces, as well the path for the files of assignments (that is, multiple file paths must be within the same double quotes). You can also specify ```-t``` if it is a tutorial instead of an assignment, so the MD file will be written accordingly, as well as specifying a different name for the output file using the ```-o``` flag, otherwise the file will be named ```week$QUARTER_WEEK.md"```.

As it is cumbersome to type your name, student number and class all the time, you can define an alias for the command with the arguments already filled. This is slightly different on each shell/Os, thus you will have to look how to set that up.

## Contributions

This is an extremely simple tool. I don't pretend to invest too much time on it, as I stitched it together over a pair of hours. If you find a bug (I am pretty sure there are some), feel free to open an issue and I look that up when I have time. If you have a suggestion, feel free to open an issue as well. If it is nice I might implement it. If you want you can fork it as well. It is under the MIT license, so do as you please with the code.
