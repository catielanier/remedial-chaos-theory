import os
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel

HOST = os.getenv("HOST", "127.0.0.1")
PORT = int(os.getenv("PORT", "5175"))

app = FastAPI(title="Tourney Flow Backend", version="0.1.0")

# Allow the Tauri app (localhost) during dev
app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:5173", "tauri://localhost"],  # vite + tauri
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

class IngestRequest(BaseModel):
    tournament_slug: str

@app.get("/health")
async def health():
    return {"ok": True}

@app.get("/version")
async def version():
    return {"version": app.version}

@app.post("/ingest")
async def ingest(body: IngestRequest):
    # TODO: you’ll add Start.gg calls + analysis later
    return {"received": body.tournament_slug, "status": "not-implemented"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("app:app", host=HOST, port=PORT, reload=True)