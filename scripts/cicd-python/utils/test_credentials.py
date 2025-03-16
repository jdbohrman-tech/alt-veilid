import aiohttp
import sys

async def test_api_credentials(token: str) -> None:
    headers = {
        "Authorization": f"Bearer {token}",
        "Content-Type": "application/json",
    }
    url = "https://api.digitalocean.com/v2/droplets"
    async with aiohttp.ClientSession() as session:
        async with session.get(url, headers=headers) as resp:
            if resp.status == 200:
                data = await resp.json()
                droplets = data.get("droplets", [])
                print("API credentials are valid.")
                print(f"Retrieved {len(droplets)} droplet(s).")
            else:
                error_text = await resp.text()
                print("Failed to authenticate API credentials:", error_text, file=sys.stderr)
                sys.exit(error_text)
