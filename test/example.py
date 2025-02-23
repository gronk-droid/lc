# spotify_app.py

from flask import Flask, jsonify, request
import requests
import os

app = Flask(__name__)

SPOTIFY_CLIENT_ID = os.getenv("SPOTIFY_CLIENT_ID")
SPOTIFY_CLIENT_SECRET = os.getenv("SPOTIFY_CLIENT_SECRET")
SPOTIFY_REDIRECT_URI = os.getenv("SPOTIFY_REDIRECT_URI")
SPOTIFY_TOKEN_URL = "https://accounts.spotify.com/api/token"
SPOTIFY_API_URL = "https://api.spotify.com/v1"


def get_access_token():
    """Get access token from Spotify API."""
    response = requests.post(
        SPOTIFY_TOKEN_URL,
        data={
            "grant_type": "client_credentials",
            "client_id": SPOTIFY_CLIENT_ID,
            "client_secret": SPOTIFY_CLIENT_SECRET,
        },
    )
    response_data = response.json()
    return response_data.get("access_token")


@app.route("/playlists", methods=["GET"])
def get_playlists():
    """Fetch user's playlists from Spotify."""
    access_token = get_access_token()
    headers = {"Authorization": f"Bearer {access_token}"}
    user_id = request.args.get("user_id")  # Get user ID from query parameters
    response = requests.get(
        f"{SPOTIFY_API_URL}/users/{user_id}/playlists", headers=headers
    )

    if response.status_code == 200:
        return jsonify(response.json())
    else:
        return jsonify({"error": "Unable to fetch playlists"}), response.status_code


if __name__ == "__main__":
    app.run(debug=True)
