import aiohttp
import asyncio
import sys
import json

CONFIG_FILE = "config.json"

# Load config from file
def load_config():
    try:
        with open(CONFIG_FILE, "r") as f:
            return json.load(f)
    except (FileNotFoundError, json.JSONDecodeError):
        return {}

# Save config to file
def save_config(config):
    with open(CONFIG_FILE, "w") as f:
        json.dump(config, f, indent=4)

async def create_build_machine(token: str) -> None:
    config = load_config()
    droplet_config = config.get("droplet_config", {})
    
    if not droplet_config:
        print("Droplet configuration not found.", file=sys.stderr)
        sys.exit(1)
    
    headers = {
        "Authorization": f"Bearer {token}",
        "Content-Type": "application/json",
    }
    create_url = "https://api.digitalocean.com/v2/droplets"
    payload = {
        "name": droplet_config["name"],
        "region": "nyc1",
        "size": droplet_config["size"],
        "image": droplet_config["image"],
        "backups": False,
    }
    
    async with aiohttp.ClientSession() as session:
        async with session.post(create_url, headers=headers, json=payload) as resp:
            if resp.status not in (201, 202):
                error_text = await resp.text()
                print(f"Error creating droplet: {error_text}", file=sys.stderr)
                sys.exit(error_text)
            data = await resp.json()
            droplet = data.get("droplet")
            if not droplet:
                print("No droplet information returned.", file=sys.stderr)
                sys.exit("No droplet information returned.")
            droplet_id = droplet.get("id")
            print(f"Droplet created. Droplet ID: {droplet_id}")
            
            # Save droplet ID to config
            config["droplet_id"] = droplet_id
            save_config(config)
            print("Droplet ID saved to config.")

            # Poll every 10 second for droplet status until it becomes "active"
            status = droplet.get("status", "new")
            droplet_url = f"https://api.digitalocean.com/v2/droplets/{droplet_id}"
            while status != "active":
                await asyncio.sleep(10)
                async with session.get(droplet_url, headers=headers) as poll_resp:
                    if poll_resp.status != 200:
                        error_text = await poll_resp.text()
                        print(f"Error polling droplet status: {error_text}",
                              file=sys.stderr)
                        sys.exit(error_text)
                    droplet_data = await poll_resp.json()
                    droplet = droplet_data.get("droplet")
                    if droplet:
                        status = droplet.get("status", status)
                        print(f"Droplet status: {status}")
                    else:
                        print("Droplet data missing in polling response",
                              file=sys.stderr)
                        sys.exit("Droplet data missing in polling response")
            
            print("Droplet is up and running.")
            # Once active, send a final GET request to output the droplet's information.
            async with session.get(droplet_url, headers=headers) as final_resp:
                if final_resp.status != 200:
                    error_text = await final_resp.text()
                    print(f"Error retrieving droplet information: {error_text}",
                          file=sys.stderr)
                    sys.exit(error_text)
                final_data = await final_resp.json()
                print("Droplet Information:")
                print(final_data)

async def delete_build_machine(token: str) -> None:
    config = load_config()
    droplet_id = config.get("droplet_id")
    
    if not droplet_id:
        print("No droplet ID found in config.", file=sys.stderr)
        return
    
    headers = {
        "Authorization": f"Bearer {token}",
        "Content-Type": "application/json",
    }
    delete_url = f"https://api.digitalocean.com/v2/droplets/{droplet_id}"
    
    async with aiohttp.ClientSession() as session:
        async with session.delete(delete_url, headers=headers) as resp:
            if resp.status != 204:
                error_text = await resp.text()
                print(f"Error deleting droplet: {error_text}", file=sys.stderr)
                sys.exit(error_text)
            print(f"Droplet {droplet_id} deleted successfully.")
            
            # Remove droplet ID from config
            config.pop("droplet_id", None)
            save_config(config)
            print("Droplet ID removed from config.")
