import requests
import json

def test_api(base_url, payload):
    url = f"{base_url}/solve"
    headers = {"Content-Type": "application/json"}
    
    response = requests.post(url, data=json.dumps(payload), headers=headers)
    
    print(f"Testing with payload: {json.dumps(payload)}")
    print(f"Status code: {response.status_code}")
    
    if response.status_code == 200:
        try:
            print("Response:")
            print(json.dumps(response.json(), indent=4))
        except json.JSONDecodeError:
            print("Response (raw):", response.text)
    else:
        print("Response:", response.text)
    
    print("\n" + "-"*40 + "\n")

if __name__ == "__main__":
    base_url = "http://127.0.0.1:8080"
    
    # Valid cases with solutions
    valid_payloads = [
        {"x_capacity": 2, "y_capacity": 10, "z_amount_wanted": 4},
        {"x_capacity": 3, "y_capacity": 5, "z_amount_wanted": 4},
        {"x_capacity": 7, "y_capacity": 3, "z_amount_wanted": 5},
        {"x_capacity": 4, "y_capacity": 100, "z_amount_wanted": 96}
    ]
    
    # Edge case with no solution
    no_solution_payload = {"x_capacity": 2, "y_capacity": 6, "z_amount_wanted": 5}
    
    # Invalid cases
    invalid_payloads = [
        {"x_capacity": -1, "y_capacity": 10, "z_amount_wanted": 4},  # Negative capacity
        {"x_capacity": 0, "y_capacity": 10, "z_amount_wanted": 4},   # Zero capacity
        {"x_capacity": 2, "y_capacity": 10, "z_amount_wanted": -4},  # Negative target amount
        {"x_capacity": 2, "y_capacity": 10, "z_amount_wanted": 15},  # Target amount greater than both capacities
        {"x_capacity": 5, "y_capacity": 5, "z_amount_wanted": 6}     # Target amount greater than any capacity
    ]
    
    # Run tests for valid cases
    print("Testing valid payloads...\n")
    for payload in valid_payloads:
        test_api(base_url, payload)
    
    # Run test for no solution case
    print("Testing no solution payload...\n")
    test_api(base_url, no_solution_payload)
    
    # Run tests for invalid cases
    print("Testing invalid payloads...\n")
    for payload in invalid_payloads:
        test_api(base_url, payload)

