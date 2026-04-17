
# DLSE ESD compiler and decompiler (Demon's Souls)

This program is a compiler and decompiler for the TalkESD files of the game Demon's Souls.
Please note that this Tool is still WIP and has barely been tested so **expect bugs**.
If you find any bugs please report them either through Github issues or in the SoulsModding ?ServerName? discord server, my username is "moitt22".


## How to use
Drag and drop the .esd file onto the .exe, it will generate a file with the extension .esd.py next to the esd file you dropped onto the exe. 
To recompile the .esd.py just dragndrop back onto the exe, it will create a file with the extension .esd, if a file with that name already exists, the existing will be renamed to <filename>.esd.bak.  
In the .esd.py file (I'll use t0615.esd as an example) you'll see something like this:
```python
def Map_1():                <- indicates that following states are part of Map_1 (Map)
    def State_0():          <- indicates that following instructions are part of State_0 (MapState)
        #Transitions:       <- indicates that the following instructions are transitions
        if True:            <- the condition of the transition
            State_17()      <- target state of transition      
    def State_1():
        #EntryEvents:       <- indicates that the following lines are entry events
        DebugEvent("待機")  <- the event
        #Transitions:
        if IsPlayerTalkingToMe() == 1 and GetRelativeAngleBetweenPlayerAndSelf() <= 45 and GetDistanceToPlayer() <= 2 and GetOneLineHelpStatus() == 1 and GetEventFlag(115) == 1:
            State_114()
        if GetEventFlag(71) == 1:
            State_100()
        if GetEventFlag(40) == 1:
            State_2()
        if GetEventFlag(16141) == 1:
            State_11()
        if GetOneLineHelpStatus() == 0 and HasDisableTalkPeriodElapsed() == 1 and IsTalkingToSomeoneElse() == 0 and CheckSelfDeath() == 0 and IsCharacterDisabled() == 0 and IsClientPlayer() == 0 and GetRelativeAngleBetweenPlayerAndSelf() <= 45 and GetDistanceToPlayer() <= 2:
            State_4()
        if IsPlayerTalkingToMe() == 1 and GetRelativeAngleBetweenPlayerAndSelf() <= 45 and GetDistanceToPlayer() <= 2 and GetOneLineHelpStatus() == 1 and GetEventFlag(8704) == 0:
            State_9()
        if GetOneLineHelpStatus() == 1 and (IsTalkingToSomeoneElse() or CheckSelfDeath() or IsCharacterDisabled() or IsClientPlayer() == 1 or GetRelativeAngleBetweenPlayerAndSelf() > 45 or GetDistanceToPlayer() > 2):
            State_5()
        if IsAttackedBySomeone() == 1:
            State_124()

```  
The python file should always have this structure:
```py
def Map_1():
    def State_x():
        #EntryEvents:   <- if there are entry events
        Event(args)
        #ExitEvents:    <- if there are exit events
        Event(args)
        #Transitions    <- if there are transitions
        if condition:
            TargetState()
        #PassEvents     <- if the transition has pass events
        Event(args)
```  
**How does the state machine work?**  
The state machine always starts with entering State_0, while the state machine enters the state it executes the EntryEvents of that State. After that it stays in that state until one of the transition's conditions is true. Once a condition is true the state machine exits the current state, on exit it executes the ExitEvents.  
If the transition whose transition is true has pass events, those will be executed too.

**Conditions**  
Transitions basically are python if conditions, so you can do something like this:
```py
if GetEventFlag(100) == 1 and (GetDistanceToPlayer() <= 5 or GetEventFlag(200) == 0):
```
but this tool doesn't support following types of conditions stable yet (the compiler is at the moment focused to work with vanilla scripts which don't have these kinds of operators):
```py
if GetEventFlag(100) == GetEventFlag(200):
if GetEventFlag(GetDistanceToPlayer()) == 1:
if GetTalkListEntryResult(1 + 2 * 4):
```
Operators that are tested are "and", "or", "==", "<=", ">=", "<", ">", "(", ")"  
Operators that might work are "not", "+", "-", "*", "/"
## Acknowledgements

 - [TKGP](https://github.com/JKAnderson) for his binary template for this format
 - [ESDLang](https://github.com/thefifthmatt/ESDLang/tree/master) documentation of Events, Functions and Condition bytecode

