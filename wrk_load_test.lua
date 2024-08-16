-- wrk -t1 -c1 -d30s -s load_test.lua https://water-bucket-challenge-production.up.railway.app
wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"

valid_payloads = {
    '{"x_capacity": 2, "y_capacity": 10, "z_amount_wanted": 4}',
    '{"x_capacity": 3, "y_capacity": 5, "z_amount_wanted": 4}',
    '{"x_capacity": 7, "y_capacity": 3, "z_amount_wanted": 5}',
    '{"x_capacity": 4, "y_capacity": 100, "z_amount_wanted": 96}'
}

no_solution_payload = '{"x_capacity": 2, "y_capacity": 6, "z_amount_wanted": 5}'

invalid_payloads = {
    '{"x_capacity": -1, "y_capacity": 10, "z_amount_wanted": 4}',
    '{"x_capacity": 0, "y_capacity": 10, "z_amount_wanted": 4}',
    '{"x_capacity": 2, "y_capacity": 10, "z_amount_wanted": -4}',
    '{"x_capacity": 2, "y_capacity": 10, "z_amount_wanted": 15}',
    '{"x_capacity": 5, "y_capacity": 5, "z_amount_wanted": 6}'
}

counter = 1

request = function()
    local payload

    if counter <= #valid_payloads then
        payload = valid_payloads[counter]
    elseif counter == #valid_payloads + 1 then
        payload = no_solution_payload
    else
        payload = invalid_payloads[counter - #valid_payloads - 1]
    end

    wrk.body = payload
    counter = counter + 1

    -- Looping the requests
    if counter > (#valid_payloads + 1 + #invalid_payloads) then
        counter = 1
    end

    return wrk.format(nil, "/solve")
end
