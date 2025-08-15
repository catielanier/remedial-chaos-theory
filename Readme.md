# remedial-chaos-theory

Python backend (FastAPI) + Tauri + Svelte frontend.

## Requirements
- Python 3.10+
- Node 18+
- Rust (for Tauri)
- (Linux) libwebkit2gtk / (Windows) Visual Studio Build Tools / (macOS) Xcode CLT as per Tauri docs.

## Setup

### 1) Backend
```bash
cd backend
python -m venv .venv
source .venv/bin/activate  # (Windows: .venv\Scripts\activate)
pip install -r requirements.txt
cp .env.example .env
python -m backend.app