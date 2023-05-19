# %%
import os
import requests
import json
from dotenv import load_dotenv


load_dotenv()  # take environment variables from .env.

# %%


class Mowers:
    def __init__(self):
        self.application_key = os.getenv("APPLICATION_KEY")
        self.secret = os.getenv("APPLICATION_SECRET")
        self.auth_uri = os.getenv("AUTH_URI")
        self.api_uri = os.getenv("HUSQVARNA_URI")
        self.is_logged_in = False
        self.bearer_token = ""

    def login(self):
        payload = {
            "grant_type": "client_credentials",
            "client_id": self.application_key,
            "client_secret": self.secret,
        }

        response = requests.post(self.auth_uri, data=payload)

        if response.status_code == 200:
            try:
                self.bearer_token = response.json()["access_token"]
                self.is_logged_in = True
                return self.is_logged_in
            except json.JSONDecodeError as err:
                raise Exception(
                    f"Error in extracting JSON from the response of {self.auth_uri}\nErr; {err}"
                )
        else:
            raise Exception(
                f"Error in HTTP request to {self.auth_uri}\nErr; {response.status_code}"
            )


# %%

if __name__ == "__main__":
    print("Running mowers.py")
    mowers = Mowers()
    result = mowers.login()
    print(result)
    print(mowers.bearer_token)

# %%
