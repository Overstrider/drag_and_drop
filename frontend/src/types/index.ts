export interface FileInfo {
  file_id: string
  original_name: string
  size: number
  upload_date: string
}

export interface UploadProgress {
  file_name: string
  progress: number
  isUploading: boolean
  cancelToken?: AbortController
} 