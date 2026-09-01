import { defineCollection, z } from 'astro:content';
import { glob } from 'astro/loaders';

const desks = defineCollection({
  loader: glob({ pattern: '**/*.md', base: './src/content/desks' }),
  schema: z.object({
    title: z.string(),
    stamp: z.string(),
    summary: z.string(),
    frozen: z.string(),
    order: z.number(),
  }),
});

export const collections = { desks };
