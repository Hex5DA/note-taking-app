<script lang="ts">
    import type { Note } from "@/lib/types";
    import { format_description } from "@/lib/utils";

    export let data: Note;
</script>

<main>
    <div id="wrapper">
        <p id="title"><b>{data.title}</b></p>
        <p>{format_description(data.content)}</p>
    </div>
</main>

<style lang="scss">

    main {
        width: 250px;
        height: 200px;

        background-color: white;
        border-radius: 15px;

        transition: transform 0.15s ease-in-out;

        box-shadow: 10px 10px black;
    }

    main:hover {
        transform: scale(1.05);
    }

    #wrapper {
        max-height: calc(100% - 10px * 2);
        max-width: calc(100% - 10px * 2);

        padding: 10px;
        padding-top: 15px;
    }

    #title {
        display: flex;
        align-items: center;
        margin: 0;
    }

</style>