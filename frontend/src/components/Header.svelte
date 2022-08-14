<script lang="ts">
    import vitelogo from "@/assets/stack/vite.svg";
    import sveltelogo from "@/assets/stack/svelte.svg";
    import rustlogo from "@/assets/stack/rust.svg";
    import actixlogo from "@/assets/stack/actix.png";
    import sasslogo from "@/assets/stack/sass.svg";
    import typescriptlogo from "@/assets/stack/typescript.svg";
    import dieselogo from "@/assets/stack/diesel.png";
</script>

<header>
    <h1>Note Taking App</h1>

    <a target="_blank" href="https://vitejs.dev/"> <img class="stack-icon" src={vitelogo} alt="Vite logo"/></a>
    <a target="_blank" href="https://svelte.dev/"> <img class="stack-icon" src={sveltelogo} alt="Svelte logo"/></a>
    <a target="_blank" href="https://www.rust-lang.org/"> <img class="stack-icon" src={rustlogo} alt="Rust logo"/></a>
    <a target="_blank" href="https://actix.rs/"> <img class="stack-icon" src={actixlogo} alt="Actix logo"/></a>
    <a target="_blank" href="https://sass-lang.com/"> <img class="stack-icon" src={sasslogo} alt="Sass logo"/></a>
    <a target="_blank" href="https://www.typescriptlang.org/"> <img class="stack-icon" src={typescriptlogo} alt="Typescript logo"/></a>
    <a target="_blank" href="https://diesel.rs/"> <img class="stack-icon" src={dieselogo} alt="Diesel logo"/></a>
</header>

<style lang="scss">
    $height: 100px;

    .stack-icon {
        width: $height * .6;
        height: $height * .6;
        margin: 20px;

        transition: transform 0.15s ease-in-out;

        -webkit-filter: drop-shadow(0 0 13px #FFF);
        filter: drop-shadow(0 0 13px #FFF);
    }

    .stack-icon:hover {
        transform: scale(1.03);
    }

    header {
        background-color: black;
        width: 100%;
        height: $height;
        margin: 0;

        display: flex;
        justify-content: center;
        align-items: center;
    }

    h1 {
        position: absolute;
        left: 20px;

        color: white;
        font-size: 30px;

        font-weight: bold;
    }
</style>