INSERT INTO public.links(slug, url)
VALUES ($1, $2)
RETURNING $table_fields;