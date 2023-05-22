# mymower
experimenting with Husqvarna automower API



## how to find the authentification token
log in to your automower account on https://developer.husqvarnagroup.cloud/applications


##  set environment variables in .env file
```
APPLICATION_KEY= # from https://developer.husqvarnagroup.cloud/applications
APPLICATION_SECRET= # from https://developer.husqvarnagroup.cloud/applications
AUTH_URI=https://api.authentication.husqvarnagroup.dev/v1/oauth2/token
USER_NAME= # your email
USER_PASSWORD= # your password
HUSQVARNA_URI=https://api.amc.husqvarna.dev/v1/mowers
```

## azure IOT
https://learn.microsoft.com/en-us/azure/iot-hub/quickstart-send-telemetry-cli

```fish
# activate PIM in portal

# install azure cli
az login
# set default tenant
az account set --subscription "xxx"

# set default location
az configure --defaults location="Norway East"
# create resource group
az group create --name ktun-iot-resourcegroup
# create hub
az iot hub create --resource-group ktun-iot-resourcegroup --name ktun-iot-hub
# create device
az iot hub device-identity create -d simDevice -n ktun-iot-hub
# simulate device
az iot device simulate -d simDevice -n ktun-iot-hub 

# 2nd CLI 
az iot hub monitor-events --output table -p all -n ktun-iot-hub

# send message
az iot device c2d-message send -d simDevice --data "Hello World" --props "key0=value0;key1=value1" -n ktun-iot-hub

# send method
az iot hub invoke-device-method --mn MySampleMethod -d simDevice -n ktun-iot-hub

# set property
az iot hub device-twin update -d simDevice --desired '{"conditions":{"temperature":{"warning":98, "critical":107}}}' -n ktun-iot-hub

```
