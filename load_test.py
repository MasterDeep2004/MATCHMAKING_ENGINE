import asyncio
import aiohttp
import random

async def add_player(session):
    skill = random.randint(0, 100)
    async with session.post('http://localhost:3000/add_player', json={'skill': skill}) as resp:
        await resp.json()

async def run_load_test():
    async with aiohttp.ClientSession() as session:
        tasks = [add_player(session) for _ in range(1000)]
        await asyncio.gather(*tasks)

if __name__ == "__main__":
    asyncio.run(run_load_test())
