# What the code does
* the program initalizes TIMER1 to trigger interrupts on each second, inits ADC0 and the LCD display
* Whenever a timer interrupt occurs, the device exits sleep mode and the the chip ambiant tempreature is read and updated inside the interrupt handler. 
* then the program shows the following animation on the LCD screen and puts the cpu to sleep mode again
```
-------------------- 
|[    ]  AWAKE     | 
|TEMP: 45          | 
-------------------- 
--------------------
|[#   ]  AWAKE     |
|TEMP: 45          |
--------------------
--------------------
|[##  ]  AWAKE     |
|TEMP: 45          |
--------------------
--------------------
|[### ]  AWAKE     |
|TEMP: 45          |
--------------------
--------------------
|[### ]  AWAKE     |
|TEMP: 45          |
--------------------
--------------------
|[####] AWAKE      |
|TEMP: 45          |
--------------------
--------------------
|[####] ASLEEP     |
|TEMP: 45          |
--------------------

```
