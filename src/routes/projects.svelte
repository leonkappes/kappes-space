<script>
import Card from "$lib/Card.svelte";

</script>
<div class="w-full h-full bg-gray-800 dark:bg-gray-700">
    <div class="flex justify-center align-top flex-col pt-6">
        <h1 class="text-white text-center text-3xl">My Projects</h1>
        <div id="project-rows" class="grid grid-cols-1 md:grid-cols-2 w-full m-auto p-14">
            <Card title="This website" description="Build with svelte(-kit) and builded as a docker image" href="https://github.com/leonkappes/kappes-space"></Card>
            <Card title="Research paper | Facharbeit" description="A research paper on websockets (German) | Meine Facharbeit zum Thema Websockets" href="https://github.com/leonkappes/websocket-paper"></Card>
            <Card title="Saltychat Troubleshooter" description="A little tool(winforms) for the saltychat Teamspeak 3 Plugin. It resolves common errors endusers faced" href="https://github.com/leonkappes/Saltytroubleshooter"></Card>
            <Card title="FXServer Manager" description="Automatically restarts an FXServer/Fivem-Server(servers provided by the community for the game Grand Theft Auto V) at configured times and restarts the server if a crash is detected" href="https://github.com/leonkappes/FXServer-Autorestart"></Card>
            <Card title="Technikapp" description="Website for managing for event-technicans with an event rooster, a inventory and user managment." href="https://github.com/leonkappes/technikapp"></Card>
            <Card title="Helmrepository" descriptio="Helmrepository for my personal projects" href="https://github.com/leonkappes/helm-charts"></Card>
        </div>
    </div>
</div>