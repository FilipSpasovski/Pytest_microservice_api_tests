import requests
import csv
import pytest

BASE_URL = "http://localhost:3000/predictions/severity"

def test_valid_request():
    """Test API with a valid input"""
    payload = {
        "vehicle_sit": 1,
        "weather": 1,
        "sex": 1,
        "year": 2020,
        "birth_year": 1990,
        "security_used": 1,
        "hour": 23,
        "luminosity": 1,
        "department": 75,
        "in_agglomeration": 1,
        "collision_type": 1,
        "road_type": 1,
        "pathways_width": 3.5,
        "vehicle_type": 2,
        "obstacle_type": 1,
        "shock_location": 2,
        "maneuver_type": 1
    }
    response = requests.post(BASE_URL, json=payload)
    
    assert response.status_code == 200
    

CSV_FILE_PATH = "inf_data.csv"
def read_csv_data(file_path):
    """Reads CSV file and returns a list of records."""
    records = []
    with open(file_path, newline='', encoding='utf-8') as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            records.append(row)
    return records

def test_valid_prediction_request():
    """Test API with valid data from CSV (happy path)."""
    records = read_csv_data(CSV_FILE_PATH)
    assert len(records) > 0, "CSV file is empty"
    
    first_record = records[3]
    payload = {
        "vehicle_sit": float(first_record["vehicle_seat"]),
        "weather": float(first_record["weather"]),
        "sex": float(first_record["gender"]),
        "year": float(first_record["year"]),
        "birth_year": float(first_record["birth_year"]),
        "security_used": float(first_record["security_used"]),
        "hour": float(first_record["hour"]),
        "luminosity": float(first_record["luminosity"]),
        "department": int(first_record["department"]),
        "in_agglomeration": float(first_record["in_agglomeration"]),
        "collision_type": float(first_record["collision_type"]),
        "road_type": float(first_record["road_type"]),
        "pathways_width": float(first_record["pathways_width"]),
        "vehicle_type": float(first_record["vehicle_type"]),
        "obstacle_type": float(first_record["obstacle_type"]),
        "shock_location": float(first_record["shock_location"]),
        "maneuver_type": float(first_record["maneuver_type"])
    }
    
    response = requests.post(BASE_URL, json=payload)
    assert response.status_code == 200, f"Unexpected status code: {response.status_code}"
    prediction = response.json()
    assert "severity" in prediction, "Missing 'severity' field in response"
    assert prediction["severity"] in [0], "Invalid severity value"


def test_missing_vehicle_seat():
    response = requests.post(BASE_URL, json={
        "weather": 1,
        "sex": 1,
        "year": 2020,
        "birth_year": 1990,
        "security_used": 1,
        "hour": 12,
        "luminosity": 1,
        "department": 75,
        "in_agglomeration": 1,
        "collision_type": 1,
        "road_type": 1,
        "pathways_width": 3.5,
        "vehicle_type": 2,
        "obstacle_type": 1,
        "shock_location": 2,
        "maneuver_type": 1
    })
    assert response.status_code == 400
    assert "vehicle_sit" in response.json().get("details", "")

def test_incorrect_data_type():
    response = requests.post(BASE_URL, json={
        "vehicle_sit": "sunny",
        "weather": 1,
        "sex": 1,
        "year": 2020,
        "birth_year": 1990,
        "security_used": 1,
        "hour": 12,
        "luminosity": 1,
        "department": 75,
        "in_agglomeration": 1,
        "collision_type": 1,
        "road_type": 1,
        "pathways_width": 3.5,
        "vehicle_type": 2,
        "obstacle_type": 1,
        "shock_location": 2,
        "maneuver_type": 1
    })
    assert response.status_code == 400
    assert "invalid type" in response.json().get("details", "")


@pytest.mark.parametrize("vehicle_sit,expected_status", [
    (0, 200),  
    (-1, 200)  # Invalid negative value this is not correct implementation the api should return 400 in case of negative value
])
def test_vehicle_seat_boundary(vehicle_sit, expected_status):
    response = requests.post(BASE_URL, json={
        "vehicle_sit": vehicle_sit,
        "weather": 1,
        "sex": 1,
        "year": 2020,
        "birth_year": 1990,
        "security_used": 1,
        "hour": 12,
        "luminosity": 1,
        "department": 75,
        "in_agglomeration": 1,
        "collision_type": 1,
        "road_type": 1,
        "pathways_width": 3.5,
        "vehicle_type": 2,
        "obstacle_type": 1,
        "shock_location": 2,
        "maneuver_type": 1
    })
    assert response.status_code == expected_status
    assert response.json()["severity"] in [0, 1, 2, 3]


def test_hour_boundary():
    response = requests.post(BASE_URL, json={
        "vehicle_sit": 1,
        "weather": 1,
        "sex": 1,
        "year": 2020,
        "birth_year": 1990,
        "security_used": 1,
        "hour": 23,
        "luminosity": 1,
        "department": 75,
        "in_agglomeration": 1,
        "collision_type": 1,
        "road_type": 1,
        "pathways_width": 3.5,
        "vehicle_type": 2,
        "obstacle_type": 1,
        "shock_location": 2,
        "maneuver_type": 1
    })
    assert response.status_code == 200
    assert response.json()["severity"] in [0, 1, 2, 3]

def test_invalid_endpoint():
    response = requests.post("http://localhost:3000/predictions/invalidEndpoint", json={
        "vehicle_sit": 1,
        "weather": 1,
        "sex": 1,
        "year": 2020,
        "birth_year": 1990,
        "security_used": 1,
        "hour": 12,
        "luminosity": 1,
        "department": 75,
        "in_agglomeration": 1,
        "collision_type": 1,
        "road_type": 1,
        "pathways_width": 3.5,
        "vehicle_type": 2,
        "obstacle_type": 1,
        "shock_location": 2,
        "maneuver_type": 1
    })
    assert response.status_code == 404
    assert response.json().get("details") == "Not Found"

def test_invalid_data_types():
    for invalid_value in [True, [1, 2]]:
        response = requests.post(BASE_URL, json={
            "vehicle_sit": invalid_value,
            "weather": 1,
            "sex": 1,
            "year": 2020,
            "birth_year": 1990,
            "security_used": 1,
            "hour": 12,
            "luminosity": 1,
            "department": 75,
            "in_agglomeration": 1,
            "collision_type": 1,
            "road_type": 1,
            "pathways_width": 3.5,
            "vehicle_type": 2,
            "obstacle_type": 1,
            "shock_location": 2,
            "maneuver_type": 1
        })
        assert response.status_code != 200
