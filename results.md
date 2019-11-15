
### Simple recursion without panics and catching of them
Simply fn. depth 1 #2   time:   [342.32 ps 344.72 ps 347.63 ps]
Simply fn. depth 233    time:   [337.78 ps 339.21 ps 340.99 ps]

### Recursion with the catching at the top, and without through panics
Catch panic at top. depth 1 #2
                        time:   [4.4041 ns 4.4234 ns 4.4462 ns]
Catch panic at top. depth 233
                        time:   [4.4057 ns 4.4191 ns 4.4333 ns]


### Recursion with the catching on every layer, and without through panics
Catch panic. depth 1 #2 time:   [5.4448 ns 5.4688 ns 5.4972 ns]
Catch panic. depth 233  time:   [2.0052 us 2.0130 us 2.0218 us]

### Recursion with the catching at the top, and through a panic at the depth
Catch panic at top, throw at bottom. depth 1 #2
                        time:   [3.3571 us 3.3723 us 3.3886 us]
Catch panic at top, throw at bottom. depth 233
                        time:   [3.3686 us 3.3988 us 3.4374 us]

### Recursion with the catching on every layer, and through a panic at the depth
Catch panic throw once. depth 1 #2
                        time:   [3.6676 us 3.6929 us 3.7218 us]
Catch panic throw once. depth 233
                        time:   [6.9818 us 7.0156 us 7.0551 us]


### Catch panics at every layer and resume it, and throw a panic at the depth
Catch panic throw. depth 1 #2
                        time:   [3.7412 us 3.7725 us 3.8129 us]
Catch panic throw. depth 233
                        time:   [996.76 us 1.0033 ms 1.0108 ms]



