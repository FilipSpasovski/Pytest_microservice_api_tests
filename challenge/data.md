# Data

## accident_id

Id of the accident.

## vehicle_seat

Seat used in the vehicle.

### car/bus

```txt
1. Front left
2. Front center
3. Front right
4. Center left
5. Center
6. Center right
7. Back left
8. Back center
9. Back right
```

### Motorbike

```txt
1. Driver
2. Passenger
3. Sidecar passenger
```

## weather

Atmospheric condition.

```txt
1. Normal
2. Light Rain
3. Heavy rain
4. Snow/hail
5. Fog/smoke
6. Heavy wind/Tempest
7. Dazzling weather
8. Cloudy weather
9. Other
```

## gender

Gender of the person.

```txt
1. Male
2. Female
```

## year

Year of the accident

## birth_year

Birth year of the person.

## security_used

Security equipment used.

```txt
1. Yes
2. No
3. Unknown
```

## hour

Hour (with decimal) of the accident.

## luminosity

Light of the accident.

```txt
1. Day
2. Dusk or daybreak
3. Night without public light
4. Night with public light off
5. Night with public light on
```

## department

Department of the accident. It is a code from INSEE (french institute). Between 0 and 999.

## in_agglomeration

Accident is in an agglomeration.

```txt
1. No
2. Yes
```

## collision_type

Type of collision.

```txt
1. 2 vehicles, front shock
2. 2 vehicles, back shock
3. 2 vehicles, side shock
4. 3 or more vehicles, back to back shock
5. 3 or more vehicles, multiple shock
6. Other type
7. Without collision
```

## road_type

Type of the road.

```txt
1. Highway
2. National road
3. Departmental road
4. Municipal road
5. Outside public roads
6. Parking open to public roads
9. Other
```

## pathways_width

Width of the road (without emergency lane, etc).

## vehicle_type

Category of the vehicle.

```txt
1. Bicycle
2. Moped
3. Cart
4. Register Scooter (not longer used since 2006)
5. Motorcycle (not longer used since 2006)
6. Sidecar (not longer used since 2006)
7. Light vehicle only
8. Light vehicle and caravan (not longer used since 2006)
9. Light vehicle and trailer (not longer used since 2006)
10. Utility vehicle only from 1.5t to 3.5t
11. Utility vehicle only from 1.5t to 3.5t and caravan (not longer used since 2006)
12. Utility vehicle only from 1.5t to 3.5t and trailer (not longer used since 2006)
13. Heavy vehicle only from 3.5t to 7.5t
14. Heavy vehicle only over 7.5t
15. Heavy vehicle only over 7.5t + trailer
16. Road tractor unit only
17. Road tractor and trailer
18. Public transit (not longer used since 2006)
19. Trolley (not longer used since 2006)
20. Special vehicle
21. Farm tractor
30. Scooter <50 cm3
31. Motorcycle> 50 cm3 and <= 125 cm3
32. Scooter> 50 cm3 and <= 125 cm3
33. Motorcycle> 125 cm3
34. Scooter> 125 cm3
35. Light quad <= 50 cm3 (Quadricycle without bodywork engine)
36. Heavy quad> 50 cm3 (Quadricycle without bodywork engine)
37. Bus
39. Train
40. Trolley
99. Other
```

## obstacle_type

Fixed obstacle hit.

```txt
1. Parked vehicle
2. Tree
3. Metal slide
4. Concrete slide
5. Other slide
6. Building, wall, bridge pier
7. Vertical signage support or emergency call station
8. Pole
9. Street furniture
10. Parapet
11. Island, refuge, upper terminal
12. Sidewalk edge
13. Ditch, embankment, rock face
14. Other fixed obstacle on the road
15. Other fixed obstacle on sidewalk or shoulder
16. Clearance of the roadway without obstacle
```

## shock_location

Shock location on the vehicle.

```txt
1. front
2. Right front
3. Front left
4. back
5. Right back
6. Left back
7. Right side
8. Left side
9. Multiple shocks (rolls)
```

## maneuver_type

Main maneuver before the accident.

```txt
1. Without change of direction
2. Same direction, same lanes
3. Between 2 lanes
4. In reverse
5. In the wrong direction
6. Crossing the central reservation
7. In the bus lane, in the same direction
8. In the bus lane, in the opposite direction
9. By inserting
10. By making a U-turn on the road
```

### Changing lane

```txt
11. Left
12. Right
```

### Deporting

```txt
13. Left
14. Right
```

### Turning

```txt
15. Left
16. Right
````

### Overtaking

```txt
17. Left
18. Right
```

### Various

```txt
19. Crossing the road
20. Parking maneuver
21. Avoidance maneuver
22. Door opening
23. Stopped (except parking)
24. Parked (with occupants)
```

## severity

Severity of the accident.

```txt
0. No injuries
1. Minor
2. Major
3. Fatal
```
