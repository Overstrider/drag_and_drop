import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"

// merge classes
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
} 