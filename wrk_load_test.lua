-- wrk -t10 -c20 -d10s -s wrk_load_test.lua https://water-bucket-challenge-production.up.railway.app
wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"

-- Define the valid payload
local payload = '{"x_capacity": 3, "y_capacity": 5, "z_amount_wanted": 4}'

request = function()
    wrk.body = payload
    return wrk.format(nil, "/solve")
end
