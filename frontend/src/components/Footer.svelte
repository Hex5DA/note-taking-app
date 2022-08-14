<footer>
    <div>
        <p>This page was created for educational purposes.</p>
        <p>It is <a href="https://gitlab.com/onlyTr3v0r/note-taking-app" target="_blank">Open Source</a> and licensed with the <a href="https://unlicense.org/" target="_blank">Unlicense</a></p>
        <p>It was made by <a href="https://github.com/onlyTr3v0r" target="_blank">Trevor</a>, a member of <a href="https://github.com/DoubleFaceProgramming" target="_blank">Doubleface</a>.</p>
    </div>
</footer>

<style lang="scss">
    $height: 70px;

    footer {
        position: absolute;
        bottom: 0;

        width: 100%;
        height: $height;

        background-color: black;
        display: flex;
        align-items: center;
    }

    p {
        font-size: $height / 3 / 2;
        color: white;
        line-height: normal - 6px;
        margin: 2px;
        margin-left: $height / 5;
    }

    a {
        color: white;
        text-decoration: none;
        border-bottom: 1px dashed white;
        font-weight: bold;
    }

    a:hover {
        border-bottom: 2px solid white;
    }

</style>