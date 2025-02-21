# Project Setup and Test Execution Playwright

## Prerequisites

Ensure the following dependencies are installed on your machine:

1. **Docker** - Install Docker from [here](https://www.docker.com/get-started/).
2. **Pyhton** - Install Python from [here](https://www.python.org/downloads/).

## Setup Instructions

1. **Clone the Repository**
   ```sh
   git clone <https://github.com/FilipSpasovski/vector8-python-tests>

2. **Open the Repository**

    Navigate to the cloned repository and open it in your preferred code editor.

3. **Start the Docker Image**

    Open a terminal in the repository's root directory and run:
    ```
    docker compose up -d
    ```
4. **Verify Microservice Status**

    Run the following command in the terminal:
    ```
    curl -X GET http://localhost:3000/status
    ```
   
## Test Setup and Execution

### Install Dependencies

Run the following commands in the terminal:
```
pip install pytest requests
pip install -r requirements.txt
```


### Run Tests
Run the following command in the terminal:
```
pytest
```

### View Test Report

Open the HTML test report using:
```
pytest --html=report.html
```

## Instructions to Run Tests on CI/CD

### Setup Instructions

1. **Clone the Repository**
    ```sh
    git clone <https://github.com/FilipSpasovski/vector8-python-tests>
    ```

2. **Open the Repository**

    Navigate to the cloned repository and open it in your preferred code editor.

3. **GitHub Actions Setup**

    Once the project is open, the editor may prompt you to install GitHub Actions as it detects the YAML file used for automation.
   
    If the prompt does not appear, manually install the GitHub Actions extension from your editor's Extension section.


### How to Run the Tests
To trigger test execution, make a change in the codebase and push the changes to the master branch. This will automatically trigger the test execution.

Tests can be re-run in the GitHub repository → **Actions** section by pressing on the **Re-run all jobs**.

### Test Report
After the test run is completed, the test report is generated and can be found under:

- GitHub Repository (vector8-python-tests) → **Actions** → Click on the test run and scroll down → Under the section **Artifacts**, the report is available for download.