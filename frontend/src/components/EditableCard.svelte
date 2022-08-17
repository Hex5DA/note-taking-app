<script lang="ts">
  import { createNote, updateNote } from "@/lib/utils.js";
  import type { OptionalNote, Note } from "@/lib/types.js";

  import cancelPath from "@/assets/icons/cancel.svg";
  import cancelHighlightedPath from "@/assets/icons/cancel_highlighted.svg";
  import savePath from "@/assets/icons/save.svg";

  import starHighlightedPath from "@/assets/icons/star_highlighted.svg";
  import starPath from "@/assets/icons/star.svg";

  import { createEventDispatcher } from "svelte";

  export let template: OptionalNote;
  let data: Note = {
    title: template.hasOwnProperty("title") ? template.title : "Title",
    content: template.hasOwnProperty("content") ? template.content : "Content",
    starred: template.hasOwnProperty("starred") ? template.starred : false,
  };

  if (Object.keys(template).length !== 0 && !("id" in template)) {
    throw new Error("Editable note with values created but no id is given.");
  }

  let dispatch = createEventDispatcher();
  let cancelHovered: boolean;

  function save_or_create() {
    if (Object.keys(template).length === 0) {
      // Create
      createNote(data);
    } else {
      // Save
      updateNote(template.id, data);
    }

    dispatch("refresh");
    dispatch("exit");
  }
</script>

<main>
  <div id="wrapper">
    <input
      id="title-field"
      class="field"
      type="text"
      contenteditable
      bind:value={data.title}
    />
    <textarea
      id="content-field"
      class="field"
      contenteditable
      rows="5"
      bind:value={data.content}
    />
    <!-- <input id="starred-field" type="checkbox" bind:checked={data.starred} /> -->

    <div id="bottom">
      <img
        id="starred-field"
        src={!data.starred ? starPath : starHighlightedPath}
        on:click={() => data.starred = !data.starred}
        alt="Star icon"
      />
      <span id="control-wrapper">
        <img
          class="control"
          src={savePath}
          alt="Save icon"
          on:click={save_or_create}
        />
        <img
          class="control"
          alt="Cancel icon"
          src={!cancelHovered ? cancelPath : cancelHighlightedPath}
          on:mouseenter={() => (cancelHovered = true)}
          on:mouseleave={() => (cancelHovered = false)}
          on:click={() => dispatch("exit")}
        />
      </span>
    </div>
  </div>
</main>

<style lang="scss">
  $padding: 10px;
  $toppadding: 15px;

  .field {
    width: calc(100% - $padding);
    border: 2px solid black;
    border-radius: 5px;
    outline: none;
    font-family: inherit;
    font-size: inherit;
  }

  #content-field {
    width: calc(100% - $padding);
    resize: none;
    margin-top: 16px;
  }

  #title-field {
    font-weight: bold;
    margin: 0;
  }

  #starred-field {
    height: 20px;
    width: 20px;
  }

  .control {
    height: 20px;
    width: 20px;

    transition: transform 0.2ms ease-in-out;
  }

  .control:hover {
    transform: scale(1.05) rotate(2deg);
  }

  #bottom {
    display: flex;
    align-items: center;

    position: absolute;
    bottom: 0;

    margin: 10px 0 10px 0;
    width: calc(100% - ($padding * 2));
  }

  #control-wrapper {
    display: inherit;
    align-items: inherit;

    position: absolute;
    right: 0;

    width: min-content;
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
    padding: $padding;
    padding-top: $toppadding;

    max-height: calc(100% - ($padding + $toppadding));
    max-width: calc(100% - $padding * 2);
    height: 100%; // I don't know if this is necessary but may as well
    width: 100%;

    position: relative;
  }
</style>
