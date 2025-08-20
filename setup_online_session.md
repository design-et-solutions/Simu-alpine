# Setup Online Session

The server will read the following two configuration files:

- `"/cfg/server_cfg.ini"`
- `"/cfg/entry_list.ini"`

Add the blacklist file which stores the client's Steam GUID who will not be allowed on the server:

- `"/blacklist.txt"`

We suggest you run the `acServer.bat` batch that will dump the log of the server directly into the exe's folder under a unique name (timestamp).

`server_cfg.ini`:

An entire section ([BOOK],[PRACTICE], etc ) can be removed: for example, if you want a race without a practice session.

```
[SERVER]

NAME=Name of the server
CARS=Car models allowed on the server (folder names in "content/cars")
TRACK=Track on the server (folder name in "content/tracks")
CONFIG_TRACK=Track subversion (folder name in "content\tracks\TRACK\ui")
MAX_CLIENTS=Max number of clients (<= track's number of pits)
RACE_OVER_TIME=Time in seconds to finish race after first car crosses finish line
ALLOWED_TYRES_OUT=Number of tyres allowed outside track before penalty
UDP_PORT=UDP port number (open in firewall)
TCP_PORT=TCP port number (open in firewall)
HTTP_PORT=Lobby port number (open in firewall for both TCP and UDP)
PASSWORD=Server password
LOOP_MODE=1 to restart from first track, 0 to disable
REGISTER_TO_LOBBY=1 to be found publicly, 0 to keep private
PICKUP_MODE_ENABLED=0 for booking mode, 1 for pickup mode
SLEEP_TIME=Do not modify
VOTING_QUORUM=Percentage of votes required for session vote to pass
VOTE_DURATION=Vote duration in seconds
BLACKLIST_MODE=0 normal kick, 1 until server restart
TC_ALLOWED=0 no TC, 1 only cars with TC, 2 any car
ABS_ALLOWED=0 no ABS, 1 only cars with ABS, 2 any car
STABILITY_ALLOWED=0 off, 1 on
AUTOCLUTCH_ALLOWED=0 off, 1 on
DAMAGE_MULTIPLIER=0 (no damage) to 100 (full damage)
FUEL_RATE=0 (no fuel usage) to XXX (100 = realistic)
TYRE_WEAR_RATE=0 (no wear) to XXX (100 = realistic)
CLIENT_SEND_INTERVAL_HZ=Server packet refresh rate (10Hz = ~100ms)
TYRE_BLANKETS_ALLOWED=1 for optimal tyre temperature after pitstop
ADMIN_PASSWORD=Admin password
QUALIFY_MAX_WAIT_PERC=Factor to calculate remaining time in qualify
WELCOME_MESSAGE=Path to welcome message file
START_RULE=0 car locked until start, 1 teleport, 2 drivethru
NUM_THREADS=Default 2
FORCE_VIRTUAL_MIRROR=1 enable for all clients, 0 optional
LEGAL_TYRES=Comma-separated list of allowed tyre short-names
MAX_BALLAST_KG=Max ballast through admin command
UDP_PLUGIN_LOCAL_PORT=Plugin local port
UDP_PLUGIN_ADDRESS=Plugin address
AUTH_PLUGIN_ADDRESS=Auth plugin address


[DYNAMIC_TRACK]

SESSION_START=% level of grip at session start
RANDOMNESS=Level of randomness at start
LAP_GAIN=Laps needed to gain 1% grip
SESSION_TRANSFER=Percentage of gained grip transferred to next session


[BOOK]
NAME=Booking session name
TIME=Session length in minutes


[PRACTICE]
NAME=Practice session name
TIME=Session length in minutes
IS_OPEN=0 closed, 1 open


[QUALIFY]
NAME=Qualify session name
TIME=Session length in minutes
IS_OPEN=0 closed, 1 open


[RACE]
NAME=Race session name
LAPS=Number of laps
WAIT_TIME=Seconds before start
IS_OPEN=0 closed, 1 open, 2 open until 20s to green light


[WEATHER_INDEX]
GRAPHICS=Folder name in "content\weather"
BASE_TEMPERATURE_AMBIENT=Ambient temperature
VARIATION_AMBIENT=Ambient temperature variation
BASE_TEMPERATURE_ROAD=Road temperature relative to ambient
VARIATION_ROAD=Variation of road temperature
```

`entry_list.ini`:

Your STEAM64 GUID can be found in your "documents/assetto corsa/logs/log.txt" just after you have run a driving session in AC.

```
[CAR_INDEX]
# Car index: from 0 to XXX
DRIVERNAME=Driver name that will appear in the lobby
TEAM=Team of the driver
MODEL=Car model (exact folder name in "content/cars")
SKIN=Car skin (exact folder name in "content/cars/MODEL/skins")
GUID=Steam64 GUID
SPECTATOR_MODE=0
BALLAST=Ballast in kg for this car
FIXED_SETUP=Setup file (empty = server uses default MODEL.ini, otherwise specify "setupname.ini")
```

**NOTE:**

- You need to insert at least a number of cars sections equal to the MAX_CLIENTS value or higher than that. So if the server is allowing 10, cars then the entry list must have cars from [CAR_0] to [CAR_9] at least.

Commandes:

- `/help`: prints the list of the available commands
- `/admin`: become administrator for the server. ex, if the password is "kunos" the command is "/admin kunos"
- `/next_session`: moves to next session
- `/restart_session`: restart the session
- `/kick`: kick a user using the rules (blacklist etc) of the server. To kick a player named "The Player": /kick The Player
- `/client_list`: shows the full car list by CAR_ID : "PlayerName"
- `/kick_id`: kick the user who occupying CAR_ID. To kick a player named "The Player" who's occupying CAR_15: /kick_id 15
- `/ballast`: add weight to the CAR_ID. To add 200 kg to the CAR_15: /ballast 15 200
- `/ban_id`: ban the user who occupying CAR_ID. To ban a player named "The Player" who's occupying CAR_15: /ban_id 15
