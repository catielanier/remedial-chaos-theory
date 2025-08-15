const BACKEND = 'http://127.0.0.1:5175' // keep in sync with backend/.env

export async function health() {
  const r = await fetch(`${BACKEND}/health`)
  return r.json()
}
export async function version() {
  const r = await fetch(`${BACKEND}/version`)
  return r.json()
}
export async function ingest(tournament_slug: string) {
  const r = await fetch(`${BACKEND}/ingest`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ tournament_slug })
  })
  return r.json()
}