export const prerender = true;

export async function GET() {
    const today = new Date().toISOString().split('T')[0];

    const urls = [
        { loc: 'https://kappes.space', changefreq: 'weekly' },
        { loc: 'https://kappes.space/projects', changefreq: 'weekly' },
        { loc: 'https://kappes.space/imprint', changefreq: 'monthly' }
    ];

    const urlEntries = urls
        .map(
            ({ loc, changefreq }) => `
        <url>
            <loc>${loc}</loc>
            <lastmod>${today}</lastmod>
            <changefreq>${changefreq}</changefreq>
        </url>`
        )
        .join('');

    return new Response(
        `<?xml version="1.0" encoding="UTF-8" ?>
<urlset
    xmlns="https://www.sitemaps.org/schemas/sitemap/0.9"
    xmlns:xhtml="https://www.w3.org/1999/xhtml"
    xmlns:mobile="https://www.google.com/schemas/sitemap-mobile/1.0"
    xmlns:news="https://www.google.com/schemas/sitemap-news/0.9"
    xmlns:image="https://www.google.com/schemas/sitemap-image/1.1"
    xmlns:video="https://www.google.com/schemas/sitemap-video/1.1"
>${urlEntries}
</urlset>`,
        {
            headers: {
                'Content-Type': 'application/xml'
            }
        }
    );
}