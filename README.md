# Kappes-space

My personal website build with sveltekit and tailwindcss. Instructions for development and building are below.
The website is hosted on https://kappes.space via a static build in a docker container.

See my [helm charts repository](https://github.com/leonkappes/helm-charts) for the helm chart used to deploy the website on kubernetes. 


## Developing

Once you've installed the dependencies with `npm install`, start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of the app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.