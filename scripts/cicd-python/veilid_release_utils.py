#!/usr/bin/env python3
import os
import sys
import argparse
import asyncio
from dotenv import load_dotenv
from utils.build_machine_control import create_build_machine, delete_build_machine
from utils.test_credentials import test_api_credentials
from utils.repos_builder import build_deb_repo, build_rpm_repo

if __name__ == "__main__":
    # Load environment variables from the .env file.
    load_dotenv()
    token = os.getenv("DO_API_TOKEN")
    if not token:
        print("Error: DO_API_TOKEN environment variable not found. Please set it in the .env file.", file=sys.stderr)
        sys.exit(1)

    # Set up command-line argument parsing.
    parser = argparse.ArgumentParser(description="Veilid compiling and releasing utility")
    parser.add_argument("--create-build-machine", action="store_true", help="Create a build machine")
    parser.add_argument("--delete-build-machine", action="store_true", help="Delete the created build machine")
    parser.add_argument("--build-deb-repo", action="store_true", help="Creates and signs .deb repository")
    parser.add_argument("--build-rpm-repo", action="store_true", help="Creates and signs .rpm repository")
    parser.add_argument("--test-api-credentials", action="store_true", help="Test DigitalOcean API credentials")
    args = parser.parse_args()

    if args.create_build_machine:
        asyncio.run(create_build_machine(token))
    elif args.delete_build_machine:
        asyncio.run(delete_build_machine(token))
    elif args.build_deb_repo:
        asyncio.run(build_deb_repo())
    elif args.build_rpm_repo:
        asyncio.run(build_rpm_repo())
    elif args.test_api_credentials:
        asyncio.run(test_api_credentials(token))
