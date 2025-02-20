# Backend software

## Build environment

To run the software, docker compose must be installed in your system.

## Run

To run the service you can use the following command

```sh
docker compose up -d
```

## Check the status

```sh
curl -X GET http://localhost:3000/status
```

Expected sample output:

```json
{
  "status": "UP",
  "version": "0.1.0",
  "start_time": "2021-07-23T13:23:14.657436700Z"
}
```

## Ask for a prediction

```sh
curl -X POST -H 'Content-Type: application/json' 'http://localhost:3000/predictions/severity' \
  -d '{ "vehicle_sit": 1, "weather": 8, "sex": 2, "year": 16, "birth_year": 1983,"security_used": 1, "hour": 14.75, "luminosity": 1, "department": 590, "in_agglomeration": 2, "collision_type": 3, "road_type": 3, "pathways_width": 0.0, "vehicle_type": 7, "obstacle_type": 7, "shock_location": 1, "maneuver_type": 1}'
```

Expected output:

```json
{ "severity": 0 }
```

The severity value can be between 0 and 3
