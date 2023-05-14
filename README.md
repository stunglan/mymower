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

