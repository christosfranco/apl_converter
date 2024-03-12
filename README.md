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

