# kappes-space

Source code for the personal website [kappes.space](https://kappes.space), built with SvelteKit and Tailwind CSS.

## Tech Stack

| Technology | Purpose |
|---|---|
| [SvelteKit](https://kit.svelte.dev/) | Application framework |
| [Svelte](https://svelte.dev/) | UI component model |
| [Tailwind CSS](https://tailwindcss.com/) | Styling |
| [TypeScript](https://www.typescriptlang.org/) | Type checking |
| [Vite](https://vitejs.dev/) | Build tooling |
| [tsparticles](https://particles.js.org/) | Particle animations |

The site is compiled to a fully static build using `@sveltejs/adapter-static` and served via nginx in a Docker container.

## Project Structure

```
kappes-space/
├── src/                  # SvelteKit application source
├── static/               # Static assets (fonts, images, etc.)
├── docker/
│   └── nginx.conf        # nginx configuration for the production container
├── .github/workflows/    # GitHub Actions CI/CD pipelines
├── Dockerfile            # Multi-stage build (deps, builder, nginx server)
├── svelte.config.js      # SvelteKit configuration (static adapter)
├── tailwind.config.cjs   # Tailwind CSS configuration
└── vite.config.ts        # Vite configuration
```

## Development

Install dependencies:

```bash
npm install
```

Start the development server:

```bash
npm run dev

# or open the app directly in a browser tab
npm run dev -- --open
```

Run type checks:

```bash
npm run check
```

## Building

Create a production build:

```bash
npm run build
```

Preview the production build locally:

```bash
npm run preview
```

## Docker

The Dockerfile uses a three-stage build:

1. **deps** installs npm dependencies on `node:22-alpine`
2. **builder** runs `npm run build` to produce the static output
3. **server** copies the build output into an `nginx:alpine` image for serving

Build and run the container locally:

```bash
docker build -t kappes-space .
docker run -p 8080:80 kappes-space
```

## Deployment

The site is deployed on Kubernetes. The GitHub Actions workflow builds and pushes the Docker image on each release. The Helm chart used for the Kubernetes deployment is maintained in the separate [helm-charts repository](https://github.com/leonkappes/helm-charts).
