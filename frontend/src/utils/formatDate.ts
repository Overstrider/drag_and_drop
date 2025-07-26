import { config } from '../config'

export function formatDate(dateString: string): string {
  const d = new Date(dateString)
  return d.toLocaleDateString(config.locale.dateFormat, {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
} 