for i in $(seq 10000 )
do
    curl -X GET localhost:8080/calculate \
        -H "Content-Type: application/json" \
        -d '[["IND", "EWR"], ["SFO", "ATL"], ["GSO", "IND"], ["ATL", "GSO"]]' &
    done
