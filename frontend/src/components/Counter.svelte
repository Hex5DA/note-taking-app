<script lang="ts">
    let clicks = 0
</script>

<main>
    <button on:click={() => clicks++}>
        Click me!
    </button>

    <p>{clicks}</p>
</main>