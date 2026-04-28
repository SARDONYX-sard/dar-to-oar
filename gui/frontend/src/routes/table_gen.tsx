import { createFileRoute } from '@tanstack/react-router';
import { TableGenEditorPage } from '@/components/templates/TableGen';

export const Route = createFileRoute('/table_gen')({
  component: TableGenEditorPage,
});
