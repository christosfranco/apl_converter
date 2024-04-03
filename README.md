# apl_converter (work in progress)
Implementing a parser and generator for APL, to Python Tensorflow, and C++ Cuda

To write apl symbols (Linux):
```
setxkbmap dk,apl -option grp:win_switch
```


Standard interpreting with gnu_apl:
```
apl -f <your_file.apl>
```

For scripting and standard IO testing: (It will just redirect output, which can then be tested.)
```
apl -f hello_world.apl --OFF --safe -s > output.txt
```

Run the apl converter with a file as input
```
cargo --build release
./target/release/apl_converter -f <path_input_file>
```


Run the apl converter with a stdin string as input
```
cargo --build release
./target/release/apl_converter -i <string_apl_program>
```

Build docker image:
```
sudo docker build -t apl_converter .
```

Run apl_converter using docker image
```
sh run_docker_image.sh <path to apl_file>
```