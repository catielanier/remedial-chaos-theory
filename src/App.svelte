<script lang="ts">
  import { health, version, ingest } from "./lib/api";
  let apiHealth: string = "…";
  let apiVersion: string = "…";
  let slug = "";

  async function ping() {
    apiHealth = JSON.stringify(await health());
    apiVersion = (await version()).version;
  }
  async function doIngest() {
    if (!slug) return;
    const res = await ingest(slug);
    alert(JSON.stringify(res, null, 2));
  }

  // ping once on load
  ping();
</script>

<main style="font-family: system-ui; padding: 16px;">
  <h1>tourney-flow</h1>
  <p>health: {apiHealth}</p>
  <p>version: {apiVersion}</p>

  <div style="margin-top: 24px;">
    <input placeholder="start.gg tournament slug" bind:value={slug} />
    <button on:click={doIngest} style="margin-left:8px;">Ingest</button>
  </div>
</main>
