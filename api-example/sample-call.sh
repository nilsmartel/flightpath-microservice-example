curl -X GET localhost:8080/calculate \
    -H "Content-Type: application/json" \
    -d '[["IND", "EWR"], ["SFO", "ATL"], ["GSO", "IND"], ["ATL", "GSO"]]'
