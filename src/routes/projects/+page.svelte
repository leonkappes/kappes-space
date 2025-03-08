<script>
	import Card from '$lib/Card.svelte';
</script>

<svelte:head>
	<title>Leon Kappes | Projects</title>
</svelte:head>
<div class="w-full h-full bg-gray-800 dark:bg-gray-700">
	<div class="flex justify-center align-top flex-col pt-6">
		<h1 class="text-white text-center text-3xl">My Projects</h1>
		<div id="project-rows" class="grid grid-cols-1 md:grid-cols-2 w-full m-auto p-14">
			<Card
				title="This website"
				description="Build with svelte(-kit) and deployed as a docker image"
				href="https://github.com/leonkappes/kappes-space"
			/>
			<Card
				title="Research paper | Facharbeit"
				description="A research paper on websockets (German) | Meine Facharbeit zum Thema Websockets"
				href="https://github.com/leonkappes/websocket-paper"
			/>
			<Card
				title="Saltychat Troubleshooter"
				description="A little tool(winforms) for the saltychat Teamspeak 3 Plugin. It resolves common errors endusers faced"
				href="https://github.com/leonkappes/Saltytroubleshooter"
			/>
			<Card
				title="FXServer Manager"
				description="Automatically restart a FXServer/Fivem-Server(servers provided by the community for the game Grand Theft Auto V) at configured times or when a crash is detected."
				href="https://github.com/leonkappes/FXServer-Autorestart"
			/>
			<Card
				title="Technikapp"
				description="Webapp for managing event-technicans with an event rooster, a inventory and user managment."
				href="https://github.com/leonkappes/technikapp"
			/>
			<Card
				title="Helmrepository"
				description="Helmrepository for my personal projects"
				href="https://github.com/leonkappes/helm-charts"
			/>
			<Card
				title="Server Infrastructure"
				description="Automation scripts to deploy my k8s cluster to hetzner"
				href="https://github.com/leonkappes/Server-Infrastructure"
			/>
			<Card
				title="Flux Repository"
				description="My GitOps kubernetes setup with fluxcd"
				href="https://github.com/leonkappes/Kubernetes-Flux"
			/>
			<div class="md:col-span-2">
				<Card
					title="Github"
					description="More projects on my github page"
					href="https://github.com/leonkappes"
				/>
			</div>
		</div>
	</div>
</div>
