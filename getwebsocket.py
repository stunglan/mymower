# %%
import websocket
import threading
import time
import mowers


# %%
def on_message(ws, message):
    print("### message ###")
    print(message)


def on_error(ws, error):
    print("### error ###")
    print(error)


def on_close(ws):
    print("### closed ###")


def on_open(ws):
    def run(*args):
        # Ping server to keep alive
        print("positions-event")
        ws.send("positions-event")
        while True:
            time.sleep(60)
            print("ping")
            ws.send("ping")

    thread = threading.Thread(target=run)
    thread.start()


def run_socket(token):
    websocket.enableTrace(True)
    ws = websocket.WebSocketApp(
        "wss://ws.openapi.husqvarna.dev/v1",
        header={"Authorization": f"Bearer {token}"},
        on_message=on_message,
        on_error=on_error,
        on_close=on_close,
    )
    ws.on_open = on_open
    ws.run_forever()


# %%
if __name__ == "__main__":
    print("Running mowers.py")
    mymower = mowers.Mowers()
    result = mymower.login()
    print(result)
    print(mymower.bearer_token)
    run_socket(mymower.bearer_token)


# %%
