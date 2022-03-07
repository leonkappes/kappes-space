<script>
import Card from "$lib/Card.svelte";
import Image from "$lib/Image.svelte";

</script>
<div class="w-full h-full bg-gray-800 dark:bg-gray-700">
    <div class="flex justify-center align-top flex-col pt-6">
        <h1 class="text-white text-center text-3xl">My Projects</h1>
        <div id="project-rows" class="grid grid-cols-2 w-full m-auto p-14">
            <Card title="This website" description="Build with svelte(-kit) and hosted as a docker image" href="https://github.com/leonkappes/kappes-space"></Card>
            <Card href="https://github.com/leonkappes/websocket-paper" title="Research paper | Facharbeit" description="A research paper on websockets (German) | Meine Facharbeit zum Thema Websockets"></Card>
            <Card title="Saltychat Troubleshooter" description="A little tool(winforms) for the saltychat Teamspeak 3 Plugin. It resolves common errors endusers faced" href="https://github.com/leonkappes/Saltytroubleshooter"></Card>
            <Card title="FXServer Manager" description="Automatically restarts an FXServer/Fivem-Server(Community provided Servers for the game Grand Theft Auto V) at configured times and restarts the server if a crash is detected"></Card>
        </div>
    </div>
</div>