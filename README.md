# starcom
Starcom is a satellite control and monitoring software.
By connecting to the desired target, Starcom will give full exposure over incoming telemetries while giving the abality to send telecommands to the target.
Its ability to extract and analyze useful data will give to the operator full control on exchanged data.
Starcom is higly customizable and scriptable, giving the possibility to automate routines useful for data collecting or spacecraft control.
Its development is thought with the space field in mind, and it offers the possibility to perform framing and deframing of the most commonly used CCSDS and ECSS standards. If you develop your own frame format, it is easy to extende Starcom to support it!

# Features

## Listening Interface
- [HIGH] Starcom should be able to connect to a serial interface, in order to read incoming telemetries and send telecommands to the spacecraft
- [MEDIUM] Starcom should be able to read a log file (compatible from Cosmos) and replay the logged telemetries and telecommands with the right timings

## Telemetry Parsing
- [HIGH] Starcom should be able to load a configuration file that is used to parse the incoming telemtries for making data human readable
- [NORMAL] Starcom should be able to laod a configuration file that defines the telecommands that can be sent to the target

## Logging
- [HIGH] Starcom should be able to log internal events happening during its execution, exporting them to a file
- [NORMAL] Starcom should be able to log all the incoming telemetries during the logging session, exporting them to a file
- [NORMAL] Starcom should be able to log all the outgoing telecommands during the logging session, exporting them to a file

## User Interaction
### CLI
- [HIGH] Starcom should be able to be run specifying the desired target
- [HIGH] Starcom should be able to be run in listening mode, where all the telemetries are displayed on the command line when received
- [NORMAL] Starcom should be able to be run in interactive mode, where all the telemetries are displayed on the command line and there is the possibility of sending telecommands to the connected target

### UI
- [HIGH] Starcom should have a UI where it is possible to select the wanted target to listen to
- [HIGH] Starcom should have a UI where all the telemetries coming from the active target are massively displayed, counting them, displaying average delay time from one another
- [HIGH] Starcom should have a UI where it is possible to inspect the data of a specific selected telemetry, updated in real-time or the abality to freeze a particular state

## Data extraction
- [HIGH] Starcom should be able to extract specific data values from telemetries in different file formats (txt, csv, etc...)
- [NORMAL] Starcom should be able to plot specific data valued fromt telemetrie and capture a screenshot of the plotted data section
- [NORMAL] Starcom should be able to deframe incoming telemetries with the most common CCSDS and ECSS standards
- [NORMAL] Starcom should be able to frame outgoing telecommands with the most common CCSDS and ECSS standards

## Scripting support
- [LOW] Starcom should be able to execute user defined scripts to automate certain actions, by listening to telemetries and executing commands