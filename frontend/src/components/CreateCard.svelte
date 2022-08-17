<script lang="ts">
  import { createEventDispatcher } from "svelte";

  let dispatcher = createEventDispatcher();
</script>

<main>
    <div id="content" on:click={() => dispatcher("new")}/>
</main>

<style lang="scss">

  main {
    width: 250px;
    height: 200px;

    background-color: rgb(175, 175, 175);
    border-radius: 15px;

    transition: transform 0.15s ease-in-out;

    border: 5px dashed black;
  }

  main:hover {
    transform: scale(1.06);
  }

  #content {
    padding: 10px;
    padding-top: 15px;

    max-height: calc(100% - (10px + 15px));
    max-width: calc(100% - 10px * 2);
    height: 100%; // I don't know if this is necessary but may as well
    width: 100%;

    position: relative;

    background-image: url("@/assets/icons/plus.svg");
    background-repeat: no-repeat;
    background-position: center;
    background-size: 40%, 40%;
  }

</style>
