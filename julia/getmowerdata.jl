# %% 
using DotEnv
using HTTP
using JSON

DotEnv.config(path="../.env")

# %%

function get_authorization(auth_uri, client_id, client_secret)
    url = auth_uri
    body = Dict(
        "client_id" => client_id,
        "client_secret" => client_secret,
        "grant_type" => "client_credentials"
    )
    r = HTTP.post(url; body=body)
    return JSON.Parser.parse(String(r.body))
end

auth = get_authorization(ENV["AUTH_URI"], ENV["APPLICATION_KEY"], ENV["APPLICATION_SECRET"])



# set up authorization headers
bearer_token = auth["access_token"]
headers = Dict("Authorization" => "Bearer $(bearer_token)", "ContentType" => "application/json")


# connect to websocket server
address = ENV["HUSQVARNA_URI"]
ws = WebSockets.open(address; headers)

token = get_token(ENV["AUTH_URI"], ENV["APPLICATION_KEY"], ENV["APPLICATION_SECRET"])



# listen for incoming messages
while true
    msg = readline(ws)
    if length(msg) > 0
        println(msg)
    end
end

# close the connection when finished
close(ws)