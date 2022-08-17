<script lang="ts">
  import { format_description, updateNote } from "@/lib/utils";
  import type { Note } from "@/lib/types";

  import deletePath from "@/assets/icons/delete.svg";
  import starPath from "@/assets/icons/star.svg";
  import deleteHighlightedPath from "@/assets/icons/delete_highlighted.svg";
  import starHighlightedPath from "@/assets/icons/star_highlighted.svg";

  export let data: Note;

  let delHovered: boolean;

  function changeStar() {
    data.starred = !data.starred;

    updateNote(data.id, {starred: data.starred});
  }
</script>

<main>
  <div id="wrapper">
    <p id="title"><b>{data.title}</b></p>
    <p>{format_description(data.content)}</p>
    <div id="control-wrapper">
      <img
        class="control"
        src={!delHovered ? deletePath : deleteHighlightedPath}
        alt="Delte icon"
        on:mouseenter={() => (delHovered = true)}
        on:mouseleave={() => (delHovered = false)}
      />
      <img
        class="control"
        src={!data.starred ? starPath : starHighlightedPath}
        on:click={changeStar}
        alt="Star icon"
      />
    </div>
  </div>
</main>

<style lang="scss">
  .control {
    height: 20px;
    width: 20px;

    transition: transform 0.2ms ease-in-out;
  }

  .control:hover {
    transform: scale(1.05) rotate(2deg);
  }

  #control-wrapper {
    position: absolute;
    bottom: 0;
    right: 0;

    display: flex;
    align-items: center;

    margin: 10px;
    gap: 8px;
  }

  main {
    width: 250px;
    height: 200px;

    background-color: white;
    border-radius: 15px;

    transition: transform 0.15s ease-in-out;

    box-shadow: 10px 10px black;
  }

  main:hover {
    transform: scale(1.03);
  }

  #wrapper {
    padding: 10px;
    padding-top: 15px;

    max-height: calc(100% - (10px + 15px));
    max-width: calc(100% - 10px * 2);
    height: 100%; // I don't know if this is necessary but may as well
    width: 100%;

    position: relative;
  }

  #title {
    display: flex;
    align-items: center;
    margin: 0;
  }
</style>
