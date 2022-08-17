<script lang="ts">
  import { onMount } from "svelte";

  import EditableCard from "@/components/EditableCard.svelte";
  import CreateCard from "@/components/CreateCard.svelte";
  import Header from "@/components/Header.svelte";
  import Footer from "@/components/Footer.svelte";
  import Card from "@/components/Card.svelte";

  import type { OptionalNote, Note } from "@/lib/types.js";
  import { fetchNotes } from "@/lib/utils.js";

  let toedit: OptionalNote | false = false;
  let notes: Note[] = [];

  async function refresh() {
    notes = await fetchNotes();
  }

  onMount(refresh);
</script>

<main>
  <Header />

  <div id="notes-wrapper">
    <CreateCard on:new={() => (toedit = {})} />

    {#if toedit !== false}
      <EditableCard
        template={toedit}
        on:exit={() => (toedit = false)}
        on:refresh={refresh}
      />
    {/if}

    {#each notes as element}
      <Card
        data={element}
        on:delete={(event) =>
          (notes = notes.filter((note) => note.id != Number(event.detail)))}
        on:edit={(event) => (toedit = event.detail)}
      />
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
    gap: 10px;
    row-gap: 30px;

    grid-template-columns: repeat(6, calc(100% / 6));
    grid-template-rows: repeat(3, calc(100% / 3));
  }
</style>
