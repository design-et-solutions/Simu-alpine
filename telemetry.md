# Données récupérables avec `ac.getCarState`

## Scalars

- SpeedMS
- SpeedMPH
- SpeedKMH
- Gas
- Brake
- Clutch
- Gear
- BestLap
- CGHeight
- DriftBestLap
- DriftLastLap
- DriftPoints
- DriveTrainSpeed
- RPM
- InstantDrift
- IsDriftInvalid
- IsEngineLimiterOn
- LapCount
- LapInvalidated
- LapTime
- LastFF
- LastLap
- NormalizedSplinePosition
- PerformanceMeter
- Steer
- TurboBoost
- Caster
- DrsAvailable
- DrsEnabled
- EngineBrake
- ERSRecovery
- ERSDelivery
- ERSHeatCharging
- ERSCurrentKJ
- ERSMaxJ
- RaceFinished
- P2PStatus
- P2PActivations

## 3D Vectors

- AccG
- LocalAngularVelocity
- LocalVelocity
- SpeedTotal
- Velocity
- WheelAngularSpeed
- WorldPosition

## 4D Vectors

- CamberRad
- CamberDeg
- SlipAngle
- SlipRatio
- Mz
- Load
- TyreRadius
- NdSlip
- TyreSlip
- Dy
- CurrentTyresCoreTemp
- ThermalState
- DynamicPressure
- TyreLoadedRadius
- SuspensionTravel
- TyreDirtyLevel

## 3D Vectors (avec <OPTIONAL_IDENTIFIER> : FL, FR, RL, RR)

- TyreContactNormal
- TyreContactPoint
- TyreHeadingVector
- TyreRightVector

## Scalars (avec <OPTIONAL_IDENTIFIER> index o)

- Aero, o=0 : drag Coefficient
- Aero, o=1 : lift Coefficient front
- Aero, o=2 : lift Coefficient rear

---

# Infos générales

- getDriverName(<CAR_ID>)
- getDriverNationCode(<CAR_ID>)
- getTrackName(<CAR_ID>)
- getTrackConfiguration(<CAR_ID>)
- getWindSpeed()
- getWindDirection()
- getTrackLength(<CAR_ID>)
- getCarName(<CAR_ID>)
- getCarSkin(<CAR_ID>)
- getLastSplits(<CAR_ID>)
- isCarInPitlane(<CAR_ID>)
- isCarInPit(<CAR_ID>)
- isAIControlled(<CAR_ID>)
- isConnected(<CAR_ID>)
- getCarBallast(<CAR_ID>)
- getCarRestrictor(<CAR_ID>)
- getCarTyreCompound(<CAR_ID>)
- getCarEngineBrakeCount(<CAR_ID>)
- getCarPowerControllerCount(<CAR_ID>)
- getCarMinHeight(<CAR_ID>)
- getServerName()
- getServerIP()
- getServerHttpPort()
- getServerSlotsCount()
- getCarsCount()
- getCarLeaderboardPosition(<CAR_ID>)
- getCarRealTimeLeaderboardPosition(<CAR_ID>)
- getCarFFB()
- setCarFFB(<VALUE>)
