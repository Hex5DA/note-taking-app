<script lang="ts">
  import { onMount } from "svelte";

  import Header from "@/components/Header.svelte";
  import Footer from "@/components/Footer.svelte";
  import Card from "./components/Card.svelte";

  import { fetchNotes } from "@/lib/utils.js";

  let notes = [];
  onMount(() => fetchNotes().then((result) => (notes = result)));
</script>

<main>
  <Header />

  <div id="notes-wrapper">
    {#each notes as element}
      <Card data={element} />
    {/each}
  </div>

  <Footer />
</main>

<style lang="scss">
  :root {
    font-family: "Inter", sans-serif;
    height: 100%;
  }

  :global(body) {
    height: 100%;
    width: 100%;
    margin: 0;

    background-color: #14181c;
  }

  #notes-wrapper {
    display: grid;
    padding: 20px;
    gap: 20px;

    grid-template-columns: repeat(6, calc(100% / 6));
    grid-template-rows: repeat(3, calc(100% / 3));
  }
</style>
