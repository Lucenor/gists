import requests
import json
from datetime import datetime

# 1. The Compliance Reality: Polling a static, lagging list
def check_cisa_kev(cve_id):
    """
    Checks CISA's KEV catalog. Fails if the vulnerability 
    has not yet completed the 41-day verification lag.
    """
    url = "https://www.cisa.gov/sites/default/files/feeds/known_exploited_vulnerabilities.json"
    try:
        response = requests.get(url, timeout=10)
        data = response.json()
        
        for vuln in data.get("vulnerabilities", []):
            if vuln["cveID"] == cve_id:
                return f"[LAGGING] Found in KEV. Added on: {vuln['dateAdded']}"
        return "[BLIND SPOT] CVE not verified by CISA yet. False sense of security."
    except Exception as e:
        return f"System Error: {e}"

# 2. The Resilience Reality: Querying independent intelligence
def query_independent_cti(cve_id, api_key="<API_KEY>"):
    """
    Queries an independent Threat Intelligence API. 
    Identifies weaponization an average of 41 days earlier.
    """
    headers = {"Authorization": f"Bearer {api_key}"}
    url = f"https://api.threatintel.example.com/v1/indicators/{cve_id}"
    
    # Simulating a real-time intel response
    simulated_response = {
        "status": "active_exploitation",
        "first_seen": "12 Oct 2024",
        "actor": "APT-29",
        "confidence": "high"
    }
    
    if simulated_response["status"] == "active_exploitation":
        return f"[PRE-MANDATE] Active threat detected. First seen: {simulated_response['first_seen']}. Initiate block."
    return "No active exploitation detected in the wild."

# Execution Output:
target_cve = "CVE-2024-XXXXX"
print(check_cisa_kev(target_cve))
# Output: [BLIND SPOT] CVE not verified by CISA yet. False sense of security.

print(query_independent_cti(target_cve))
# Output: [PRE-MANDATE] Active threat detected. First seen: 12 Oct 2024. Initiate block.
