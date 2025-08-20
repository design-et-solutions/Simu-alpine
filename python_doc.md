# Python Documentation

`import ac`

`-ac.getCarState(<CAR_IDENTIFIER>, <INFO_IDENTIFIER> , /*OPTIONAL*/ <OPTIONAL_IDENTIFIER>)`:
returns the `<INFO_IDENTIFIER>` type of information associated to car` <CAR_IDENTIFIER>`.

The optional identifier can be omitted, it is used for special infos where they require a specific tyre, as described in the following section.
The `<OPTIONAL_IDENTIFIER>` and it can be one of the following values:

```
FL,				Front Left
FR,				Front Right
RL,				Rear Left
RR,				Rear Right
```

Using the following `<INFO_IDENTIFIER>s` `ac.getCarState` returns a scalar value:

```
SpeedMS,			        Current speed using Meters/Seconds [0, …]
SpeedMPH,			        Current speed using Miles/Hour [0, …]
SpeedKMH,			        Current speed using Kilometers/Hour [0, …]
Gas,				        Pression on the Gas pedal [0,1]
Brake,				        Pression on the Brake pedal [0,1]
Clutch,				        Pression on the Clutch pedal [0,1]
Gear,				        Current Gear [0,max gear]
BestLap,			        Best Lap in milliseconds [0, …]
CGHeight,			        Height of the center of gravity of the car from the ground [0, …]
DriftBestLap,			    Best Lap points in Drift mode [0, …]
DriftLastLap,			    Last Lap points in Drift mode [0, …]
DriftPoints,			    Current Lap points in Drift mode [0, …]
DriveTrainSpeed,		    Speed Delivered to the wheels [0, …]
RPM,				        Engine’s rounds per minute [0, …]
InstantDrift,			    Current drift points in Drift Mode [0, …]
IsDriftInvalid,			    Current Drift is valid/invalid in Drift Mode {0,1}
IsEngineLimiterOn,		    Engine Limiter On/Off {0,1}
LapCount,			        Current Session Lap count [0, …]
LapInvalidated,		        Is current Lap invalidated (by going out on the grass) {0, 1}
LapTime, 			        Current LapTime in milliseconds [0, …]
LastFF,			            Last Force Feedback signal sent to the Wheel [0, …]
LastLap,			        Last Lap in milliseconds [0, …]
NormalizedSplinePosition,	Position of the car on the track in normalized [0,1]
PerformanceMeter,		    Projection of how many seconds is the current time far from the current best lap [0, …]
Steer,				        Radians of steer rotation [-2pi,2pi]
TurboBoost,			        Turbo gain on engine torque for specific vehicles [0,...]
Caster,				        Caster Angle in radians
DrsAvailable			    Gets if DRS is available in the current spline position
DrsEnabled			        Gets if DRS is enabled by the user
EngineBrake			        Gets current EngineBrake level
ERSRecovery		            Gets current ERS Recovery level
ERSDelivery			        Gets current ERS Delivery level
ERSHeatCharging		        Gets current ERS Mode
ERSCurrentKJ		        Gets current KERS/ERS used KJ
ERSMaxJ			            Gets current KERS/ERS max J for lap
RaceFinished                Gets if the car has finished the session (0, 1)
P2PStatus                   Gets current P2P status (OFF = 0, COOLING = 1, AVAILABLE = 2, ACTIVE = 3)
P2PActivations              Gets the P2P available activations
```

Using the following `<INFO_IDENTIFIER>s` `ac.getCarState` returns a 3D vector (with x,y,z components):

```
AccG,				    Gravity acceleration on the vehicle’s GC x,y,z = [0, …]
LocalAngularVelocity,	Gets the angular velocity of the car, using the car as origin x,y,z =[0, …]
LocalVelocity,			Gets the velocity using the car as origin x,y,x = [0, …]
SpeedTotal,		        Gets all the speed representation x= kmh, y = mph, z = ms
Velocity,			    Current velocity vector x,y,z = [0, …]
WheelAngularSpeed,		Current Wheel angular speed x,y,z =  [0, …]
WorldPosition			Current Car Coordinates on map x,y,z =  [0,...]
```

Using the following `<INFO_IDENTIFIER>s` `ac.getCarState` returns a 4D vector (with w,x,y,z components):

```
CamberRad,			    The camber angle in Radiants for each tyre
CamberDeg,			    The camber angle in Degree for each tyre
SlipAngle,			    Slip angle, angle between the desired direction and the actual direction of the vehicle [0, 360], degrees.
SlipRatio,			    Slip Ration of the tyres
Mz,				        Self Aligning Torque x,y,z,w = [0, …]
Load,				    Current load on each tyre x,y,z,w = [0,...]
TyreRadius,			    Radius of any Tyre x,y,z,w = [0,...]
NdSlip,				    Dimensionless lateral slip friction for each tyre
TyreSlip,			    Slip Factor for each tyre
Dy,				        Lateral friction coefficient for each tyre
CurrentTyresCoreTemp	Current core temperature of the tyres °C x,y,z,w = [0,...]
ThermalState,			Current temperature of the tyres °C x,y,z,w = [0,...]
DynamicPressure,		Current pressure of the tyres psi for each tyre = [0, …]
TyreLoadedRadius,		Radius of the tyre under load for each tyre = [0, …]
SuspensionTravel,		Suspension vertical travel x,y,z,w =[0, …]
TyreDirtyLevel,		    Quantity of dirt on the tyres x,y,z,w =[0, 10)
```

Using the following `<INFO_IDENTIFIER>s` combined with the `<OPTIONAL_IDENTIFIER>`
`ac.getCarState` returns a 3D vector (with x,y,z components) related to the `<OPTIONAL_IDENTIFIER>` wheel:

```
TyreContactNormal,		Normal vector to tyre’s contact point (z)
TyreContactPoint,		Tyre contact point with the tarmac
TyreHeadingVector,		Tyre Heading Vector (x)
TyreRightVector,		Tyre Right Vector (y)
```

Using the following `<INFO_IDENTIFIER>s` combined with the `<OPTIONAL_IDENTIFIER>` `ac.getCarState` returns a scalar vector (with x,y,z components) related to the `<OPTIONAL_IDENTIFIER>` index O:

```
Aero , o=0			drag Coefficient
Aero,  o=1			lift Coefficient front
Aero,  o=2			lift Coefficient rear
```

## GENERAL INFO

- `ac.getDriverName(<CAR_ID>)`
- `ac.getDriverNationCode(<CAR_ID>)`
- `ac.getTrackName(<CAR_ID>)`
- `ac.getTrackConfiguration(<CAR_ID>)`
- `ac.getWindSpeed()`
- `ac.getWindDirection()`
- `ac.getTrackLength(<CAR_ID>)`
- `ac.getCarName(<CAR_ID>)`
- `ac.getCarSkin(<CAR_ID>)`
- `ac.getLastSplits(<CAR_ID>)`
- `ac.isCarInPitlane(<CAR_ID>)`
- `ac.isCarInPit(<CAR_ID>)`
- `ac.isAIControlled(<CAR_ID>)`
- `ac.isConnected(<CAR_ID>)`
- `ac.getCarBallast(<CAR_ID>)`
- `ac.getCarRestrictor(<CAR_ID>)`
- `ac.getCarTyreCompound(<CAR_ID>)`
- `ac.getCarEngineBrakeCount(<CAR_ID>)`
- `ac.getCarPowerControllerCount(<CAR_ID>)`
- `ac.getCarMinHeight(<CAR_ID>)`
- `ac.getServerName()`
- `ac.getServerIP()`
- `ac.getServerHttpPort()`
- `ac.getServerSlotsCount()`
- `ac.getCarsCount()`
- `ac.getCarLeaderboardPosition(<CAR_ID>)`
- `ac.getCarRealTimeLeaderboardPosition(<CAR_ID>)`
- `ac.getCarFFB()`
- `ac.setCarFFB(<VALUE>)`

## CAMERA MANAGEMENT

- `ac.setCameraMode(<INFO_IDENTIFIER>)`
- `ac.getCameraMode()`
- `ac.isCameraOnBoard(<CAR_ID>)`
- `ac.setCameraCar(<CAMERA_ID>,<CAR_ID>)`
- `ac.getCameraCarCount(<CAR_ID>)`
- `ac.focusCar(<CAR_ID>)`
- `ac.getFocusedCar()`

## DEBUG

- `ac.log(<VALUE>)`
- `ac.console(<VALUE>)`
